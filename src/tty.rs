use axhal::console::UART;
use axsignal::signal_no::SignalNo;
use crate::signal::send_signal_to_pg;

pub fn init_uart_irq() {
    axlog::error!("[init_uart_irq] uart_irq_num: {}", axhal::platform::irq::UART_IRQ_NUM);
    axhal::irq::register_handler(axhal::platform::irq::UART_IRQ_NUM, uart_irq_handler);
    axhal::irq::set_enable(axhal::platform::irq::UART_IRQ_NUM, true);
}

/// UART IRQ Handler
pub fn uart_irq_handler() {
    let mut inner = UART.inner.lock();
    let is_receive_interrupt = inner.is_receive_interrupt();
    if is_receive_interrupt {
        inner.ack_interrupts();
        while let Some(c) = inner.getchar() {
            UART.buffer.lock().push(c);
            // crtl + c
            if c == 3 {
                send_signal_to_pg(SignalNo::SIGINT as isize);
            }
        }
    }   
}