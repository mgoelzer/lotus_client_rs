pub mod deal_proposal;

#[cfg(test)]
mod tests {
    fn vectors_equal<T: PartialEq>(a: &[T], b: &[T]) -> bool {
        let matching = a.iter().zip(b).filter(|&(a, b)| a == b).count();
        matching==a.len() && a.len()==b.len()
    }
    
    #[test]
    fn test_base64_vector1() {
        let input_base64 = "gYGCi9gqWCgAAYHiA5IgIM0ZDnOXrYczT9jfC/47iVtWrdzqgFiPGBD8FHO9SmI9GiAAAAD0WDEDrrTNgZqwjUdpSHUXxGAVFAbc5OFUm5JXCBf3cF1dvhB4TyaOB4DO6ODkR3pm5iRXQwCVEWAZK7gZZTxGAAkUlMYARQAfwAAAQFhhAqeOt8l8xFd6wRXsMP7+nPwIxWfVTVpjZQxc2DAdrXxSIWXPT2W9H7JYGS1eI/jCGgqmFuxfyKCGb4MRgcB+3PuJ00mPZHqli/jeBR8ug44vXmHNwb5m2QdRASRrca8xZg==";
        let dealprop = crate::cbor::deal_proposal::decode_storage_deal(input_base64).unwrap();
        //println!("dealprop={:?}",dealprop);
        assert!(vectors_equal(&dealprop.piece_cid, &[0, 1, 129, 226, 3, 146, 32, 32, 205, 25, 14, 115, 151, 173, 135, 51, 79, 216, 223, 11, 254, 59, 137, 91, 86, 173, 220, 234, 128, 88, 143, 24, 16, 252, 20, 115, 189, 74, 98, 61]));
    }
    #[test]
    fn test_base64_vector2() {
        let input_base64 = "gYGCi9gqWCgAAYHiA5IgIIrpVS7tcUKdcS5iZhMHSdOV0Xbq9SJdESIEJNd77VsIGgACAAD0WDEDg+33osjEKbpE2cnRG+8R5E6E2zFNy6hRND4wAFBOEy6Hv8Xm8msHip3qzN8+MXvMQwCgGmAZLtQZmqREAB6EgEQAAfwAQFhhAojvOzIRjLaQCYrNjPrZNLB/5alSskFRD8jv3HQ7dK/7iSwPPbvJE49k82J+FltbYxDSA4baR0dWxaV3Y/VQkLCHFWfbCDq1Emza5YqWUGGQ06hli+B+Ax9lcD/c3IResQ==";
        let dealprop = crate::cbor::deal_proposal::decode_storage_deal(input_base64).unwrap();
        //println!("dealprop={:?}",dealprop);
        assert!(vectors_equal(&dealprop.piece_cid, &[0, 1, 129, 226, 3, 146, 32, 32, 138, 233, 85, 46, 237, 113, 66, 157, 113, 46, 98, 102, 19, 7, 73, 211, 149, 209, 118, 234, 245, 34, 93, 17, 34, 4, 36, 215, 123, 237, 91, 8]));
    }

    #[test]
    fn test_get_piece_cid_as_str() {
        let input_base64 = "gYGCi9gqWCgAAYHiA5IgIBQE6FIAy641u8U9IAdzzPYlKrqQzmo4OmPRQRDy0AQFGggAAAD0VQHh5IHxtn/1yPh3sahOqrXKYJiP2EMAoBpgGT82GcMkRQAdzWUARQAH8AAAQFhCAeb43f4jnZJ1KAG/MdnFNrfZ+CZrhe6Q6WrTJAzRkVPGGSTLBv7oKuZYKtgW6YL760ghwEhK+W3uYzm4cAIGSxkB";
        let mut dealprop = crate::cbor::deal_proposal::decode_storage_deal(input_base64).unwrap();
        assert_eq!(dealprop.piece_cid[0], 0);
        assert_eq!(dealprop.piece_cid[1], 1);
        dealprop.piece_cid[0] = 0x08 as u8;
        dealprop.piece_cid[1] = 0x0c as u8;
        let s = dealprop.get_piece_cid_as_str();
        println!("{:?}",&dealprop);
        println!(">>>> {}",s);
    }
}
