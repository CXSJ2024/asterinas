// SPDX-License-Identifier: MPL-2.0

use core::hint::spin_loop;

use alloc::{boxed::Box, fmt::Debug, string::ToString, sync::Arc, vec::Vec};
use aster_console::{AnyConsoleDevice, ConsoleCallback};
use aster_frame::{
    io_mem::IoMem,
    sync::SpinLock,
    trap::TrapFrame,
    vm::{DmaDirection, DmaStream, HasDaddr, VmAllocOptions, VmReader, VmWriter},
};
use aster_util::safe_ptr::SafePtr;
use log::debug;

use crate::{
    device::{console::config::ConsoleFeatures, VirtioDeviceError},
    queue::VirtQueue,
    transport::VirtioTransport,
};

use super::{config::VirtioConsoleConfig, DEVICE_NAME};

pub struct ConsoleDevice {
    config: SafePtr<VirtioConsoleConfig, IoMem>,
    transport: Box<dyn VirtioTransport>,
    receive_queue: SpinLock<VirtQueue>,
    transmit_queue: SpinLock<VirtQueue>,
    send_buffer: DmaStream,
    receive_buffer: DmaStream,
    callbacks: SpinLock<Vec<&'static ConsoleCallback>>,
}

impl AnyConsoleDevice for ConsoleDevice {
    fn send(&self, value: &[u8]) {
        let mut transmit_queue = self.transmit_queue.lock_irq_disabled();
        let mut reader = VmReader::from(value);

        while reader.remain() > 0 {
            let mut writer = self.send_buffer.writer().unwrap();
            let len = writer.write(&mut reader);
            self.send_buffer.sync(0..len).unwrap();

            transmit_queue
                .add_dma_buf(&[&(self.send_buffer.daddr(), len)], &[])
                .unwrap();
            if transmit_queue.should_notify() {
                transmit_queue.notify();
            }
            while !transmit_queue.can_pop() {
                spin_loop();
            }
            transmit_queue.pop_used().unwrap();
        }
    }

    fn recv(&self, buf: &mut [u8]) -> Option<usize> {
        let mut receive_queue = self.receive_queue.lock_irq_disabled();
        if !receive_queue.can_pop() {
            return None;
        }
        let (_, len) = receive_queue.pop_used().unwrap();
        self.receive_buffer.sync(0..len as usize).unwrap();

        let recv_len = {
            // FIXME: If the length of the recv_buffer exceeds that of the buf,
            // there will be some content that is not received.
            let mut writer = VmWriter::from(buf);
            let mut reader = self.receive_buffer.reader().unwrap().limit(len as usize);
            writer.write(&mut reader)
        };

        receive_queue
            .add_dma_buf(&[], &[&self.receive_buffer])
            .unwrap();
        if receive_queue.should_notify() {
            receive_queue.notify();
        }
        Some(recv_len)
    }

    fn register_callback(&self, callback: &'static ConsoleCallback) {
        self.callbacks.lock().push(callback);
    }

    fn handle_irq(&self) {
        let mut receive_queue = self.receive_queue.lock_irq_disabled();
        if !receive_queue.can_pop() {
            return;
        }
        let (_, len) = receive_queue.pop_used().unwrap();
        self.receive_buffer.sync(0..len as usize).unwrap();

        let callbacks = self.callbacks.lock_irq_disabled();

        for callback in callbacks.iter() {
            let reader = self.receive_buffer.reader().unwrap().limit(len as usize);
            callback(reader);
        }
        receive_queue
            .add_dma_buf(&[], &[&self.receive_buffer])
            .unwrap();
        if receive_queue.should_notify() {
            receive_queue.notify();
        }
    }
}

impl Debug for ConsoleDevice {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("ConsoleDevice")
            .field("config", &self.config)
            .field("transport", &self.transport)
            .field("receive_queue", &self.receive_queue)
            .field("transmit_queue", &self.transmit_queue)
            .finish()
    }
}

impl ConsoleDevice {
    pub fn negotiate_features(features: u64) -> u64 {
        let mut features = ConsoleFeatures::from_bits_truncate(features);
        // A virtio console device may have multiple ports, but we only use one port to communicate now.
        features.remove(ConsoleFeatures::VIRTIO_CONSOLE_F_MULTIPORT);
        features.bits()
    }

    pub fn init(mut transport: Box<dyn VirtioTransport>) -> Result<(), VirtioDeviceError> {
        let config = VirtioConsoleConfig::new(transport.as_ref());
        const RECV0_QUEUE_INDEX: u16 = 0;
        const TRANSMIT0_QUEUE_INDEX: u16 = 1;
        let receive_queue =
            SpinLock::new(VirtQueue::new(RECV0_QUEUE_INDEX, 2, transport.as_mut()).unwrap());
        let transmit_queue =
            SpinLock::new(VirtQueue::new(TRANSMIT0_QUEUE_INDEX, 2, transport.as_mut()).unwrap());

        let send_buffer = {
            let mut options = VmAllocOptions::new(1);
            options.is_contiguous(true);
            let vm_segment = options.alloc_contiguous().unwrap();
            DmaStream::map(vm_segment, DmaDirection::ToDevice, false).unwrap()
        };

        let receive_buffer = {
            let mut options = VmAllocOptions::new(1);
            options.is_contiguous(true);
            let vm_segment = options.alloc_contiguous().unwrap();
            DmaStream::map(vm_segment, DmaDirection::FromDevice, false).unwrap()
        };

        let mut device = Self {
            config,
            transport,
            receive_queue,
            transmit_queue,
            send_buffer,
            receive_buffer,
            callbacks: SpinLock::new(Vec::new()),
        };

        let mut receive_queue = device.receive_queue.lock();
        receive_queue
            .add_dma_buf(&[], &[&device.receive_buffer])
            .unwrap();
        if receive_queue.should_notify() {
            receive_queue.notify();
        }
        drop(receive_queue);
        device
            .transport
            .register_queue_callback(RECV0_QUEUE_INDEX, Box::new(handle_console_input), false)
            .unwrap();
        device
            .transport
            .register_cfg_callback(Box::new(config_space_change))
            .unwrap();
        device.transport.finish_init();

        aster_console::register_device(DEVICE_NAME.to_string(), Arc::new(device));

        Ok(())
    }
}

fn handle_console_input(_: &TrapFrame) {
    aster_console::get_device(DEVICE_NAME).unwrap().handle_irq();
}

fn config_space_change(_: &TrapFrame) {
    debug!("Virtio-Console device configuration space change");
}
