use std::io::Write;
use std::io::Read;
use hidapi::HidApi;

pub struct FidoKeyHid {
    pub device: hidapi::HidDevice,    
}

impl FidoKeyHid {

    pub fn new(params: &[crate::HidParam])->Result<FidoKeyHid,&'static str> {
        let api = HidApi::new().expect("Failed to create HidApi instance");
        for param in params {
            if let Some(dev_info) = FidoKeyHid::get_path(&api, &param, 0xf1d0) {
                if let Ok(dev) = api.open_path(&dev_info.path()) {
                    let result = FidoKeyHid {
                        device: dev,
                    };
                    return Ok(result);
                }
            }    
        }
        Err("Failed to open device")
    }

    fn get_path(
        api: &hidapi::HidApi,
        param: &crate::HidParam,
        usage_page: u16,
    ) -> Option<hidapi::DeviceInfo> {
        let devices = api.device_list();
        for x in devices.cloned() {
            if x.vendor_id() == param.vid && x.product_id() == param.pid && x.usage_page() == usage_page
            {
                return Some(x);
            }
        }
        None
    }

}

impl Write for FidoKeyHid {
    fn write(&mut self, cmd: &[u8]) -> Result<usize,std::io::Error> {
        Ok(self.device.write(cmd).unwrap())
    }
    fn flush(&mut self) -> Result<(),std::io::Error> {
        Ok(())
    }
}

impl Read for FidoKeyHid {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, std::io::Error> {
        Ok(self.device.read(&mut buf[..]).unwrap())
    }
}