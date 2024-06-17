


mod ffi;


mod nftset {
    #![allow(dead_code)]
    use std::{ffi::CString, net::IpAddr, os::raw::{c_int, c_uchar, c_ulong}};
    use either::Either;

    fn to_c_addr(ip_addr: IpAddr) -> Either<[u8; 4], [u8; 16]> {
        match ip_addr {
            IpAddr::V4(ip) => {
                let octets = ip.octets();
                let mut result: [c_uchar; 4] = [0; 4];
                for i in 0..4 {
                    result[i] = octets[i] as c_uchar;
                }
                Either::Left(result)
            },
            IpAddr::V6(ip) => {
                let segments = ip.segments();
                let mut result: [c_uchar; 16] = [0; 16];
                for (i, &segment) in segments.iter().enumerate() {
                    result[i * 2] = (segment >> 8) as c_uchar;
                    result[i * 2 + 1] = (segment & 0xFF) as c_uchar;
                }
                Either::Right(result)
            },
        }
    }

    pub fn add(family_name: &str,
        table_name: &str,
        set_name: &str,
        addr: IpAddr,
        addr_len: u8,
        timeout: u64) -> anyhow::Result<i32> {
        
        let family_name = CString::new(family_name)?;
        let table_name = CString::new(table_name)?;
        let set_name = CString::new(set_name)?;
        
        let addr = to_c_addr(addr);
        let addr = match addr.as_ref() {
            Either::Left(v) => v.as_ptr(),
            Either::Right(v) => v.as_ptr(),
        };

        unsafe {
            Ok(crate::ffi::nftset::nftset_add(
                family_name.as_ptr(), 
                table_name.as_ptr(), 
                set_name.as_ptr(), 
                addr, 
                addr_len as c_int, 
                timeout as c_ulong
            ) as i32)
        }
    }


    pub fn del(
        family_name: &str,
        table_name: &str,
        set_name: &str,
        addr: IpAddr,
        addr_len: u8,
    ) -> anyhow::Result<i32> {
        let family_name = CString::new(family_name)?;
        let table_name = CString::new(table_name)?;
        let set_name = CString::new(set_name)?;
        
        let addr = to_c_addr(addr);
        let addr = match addr.as_ref() {
            Either::Left(v) => v.as_ptr(),
            Either::Right(v) => v.as_ptr(),
        };

        unsafe {
            Ok(crate::ffi::nftset::nftset_del(
                family_name.as_ptr(), 
                table_name.as_ptr(), 
                set_name.as_ptr(), 
                addr, 
                addr_len as c_int,
            ) as i32)
        }
    }


}


fn print_string(s: &str) -> anyhow::Result<()>  {
    use std::ffi::CString;
    let c_string = CString::new(s)?;
    unsafe {
        ffi::foo::print_string(c_string.as_ptr());
    }
    Ok(())
}


fn main() {
    let ret = nftset::add("inet", "ray", "directlist_v4", "192.168.0.0".parse().unwrap(), 16, 0).unwrap();
    println!(">>>>>>> {ret}");
    print_string("hello123 ").unwrap();
}