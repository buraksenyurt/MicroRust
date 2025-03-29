#![no_std]
#![no_main]

use core::ptr::write_volatile;
use cortex_m::asm::nop;
use cortex_m_rt::entry;
use panic_halt as _;

#[entry]
fn start() -> ! {
    
    const GPIO0_BASE: u32 = 0x5000_0000; // GPIO0 modülünün başlangıç adresi
    const PIN_CNF_OFFSET: u32 = 0x700; // GPIO0_PIN_CNF_21 ve GPIO0_PIN_CNF_28 pinlerinin konfigürasyon adresinin başlangıç adresi
    const P0_21: usize = 21; // GPIO0_PIN_CNF_21 pininin numarası
    const P0_28: usize = 28; // GPIO0_PIN_CNF_28 pininin numarası
    const GPIO0_PIN_CNF_21_ROW_1_ADDR: *mut u32 =
        (GPIO0_BASE + PIN_CNF_OFFSET + (P0_21 * 4) as u32) as *mut u32; // GPIO0_PIN_CNF_21 pininin konfigürasyon adresi
    const GPIO0_PIN_CNF_28_COL_1_ADDR: *mut u32 =
        (GPIO0_BASE + PIN_CNF_OFFSET + (P0_28 * 4) as u32) as *mut u32; // GPIO0_PIN_CNF_28 pininin konfigürasyon adresi
    const DIRECTION_OUTPUT_POS: u32 = 0;
    const PIN_CNF_DRIVE_LED: u32 = 1 << DIRECTION_OUTPUT_POS; // GPIO0_PIN_CNF_21 ve GPIO0_PIN_CNF_28 pinlerinin çıkış yönünü belirtir
    // GPIO0_PIN_CNF_21 ve GPIO0_PIN_CNF_28 pinlerini çıkış olarak ayarlar

    unsafe { // Güvenli olmayan kod bloğu
        write_volatile(GPIO0_PIN_CNF_21_ROW_1_ADDR, PIN_CNF_DRIVE_LED); // GPIO0_PIN_CNF_21 pinini çıkış olarak ayarlar
        write_volatile(GPIO0_PIN_CNF_28_COL_1_ADDR, PIN_CNF_DRIVE_LED); // GPIO0_PIN_CNF_28 pinini çıkış olarak ayarlar
    }
    const GPIO0_OUTPUT_ADDRESS: *mut u32 = (GPIO0_BASE + 4) as *mut u32; // GPIO0 modülünün çıkış adresi
    const GPIO0_OUTPUT_ROW_1_POS: u32 = 21; // GPIO0_OUTPUT_ADDRESS adresinde hangi bitin LED'i kontrol ettiğini belirtir
    let mut light_is_on: bool = false;
    loop { 
        unsafe { // Güvenli olmayan kod bloğu
            write_volatile(
                GPIO0_OUTPUT_ADDRESS,
                (light_is_on as u32) << GPIO0_OUTPUT_ROW_1_POS,
            ); // GPIO0_OUTPUT_ADDRESS adresine yazma işlemi yaparak LED'i yakıp söndürür
            for _ in 0..400_000 { // Yaklaşık 1 saniyelik gecikleme süresi
                nop(); // No-operation döngüsü
            }
            light_is_on = !light_is_on;
        }
    }
}
/*
    Bu program doğrudan GPIO0 modülünün bellek adreslerine erişerek ortadaki LED'i kontrol eder ve onu saniyede bir yakıp söndürür.
    
*/