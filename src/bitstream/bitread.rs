use bitstream::byteread::*;


pub trait BitReadEndian {
    fn peek_val(&mut self, n:usize) -> u64;
    fn merge_val(msp:u64, lsp:u64, msb:usize, lsb:usize) -> u64;
    fn skip_rem(&mut self, n:usize) -> ();
    fn fill32(&self) -> u64;
    fn fill64(&self) -> u64;
}

pub trait BitReadInternal : BitReadEndian {
    #[inline]
    fn left(&self) -> usize;
    fn refill32(&mut self) -> ();
    fn refill64(&mut self) -> ();

    fn get_val(&mut self, n:usize) -> u64 {
        let ret = self.peek_val(n);

        self.skip_rem(n);

        return ret;
    }

}

pub trait BitRead<'a>: BitReadInternal+Clone {
    fn new(&'a[u8]) -> Self;
    fn consumed(&self) -> usize;
    fn available(&self) -> usize;
    fn can_refill(&self) -> bool;

    fn skip_bits(&mut self, size : usize) -> ();

    #[inline]
    fn get_bit(&mut self) -> bool {
        if self.left() <= 0 {
            self.refill64();
        }

        self.get_val(1) != 0
    }

    #[inline]
    fn get_bits_64(&mut self, mut n:usize) -> u64 {
        let mut left = 0;
        let mut ret = 0;

        if n == 0 {
            return 0;
        }

        if self.left() < n {
            n   -= self.left();
            left = self.left();
            ret  = self.get_val(left);
            self.refill64();
        }

        Self::merge_val(self.get_val(n), ret, left, n)
    }

    #[inline]
    fn get_bits_32(&mut self, n:usize) -> u32 {
        if n == 0 {
            return 0;
        }

        if self.left() <= n {
            self.refill32();
        }

        return self.get_val(n) as u32;
    }


    #[inline]
    fn peek_bits_32(&mut self, n:usize) -> u32 {
        if n == 0 {
            return 0;
        }

        if self.left() <= n {
            self.refill32();
        }

        return self.peek_val(n) as u32;
    }

    #[inline]
    fn peek_bits_64(&mut self, n:usize) -> u64 {
        let mut tmp = self.clone();

        tmp.get_bits_64(n)
    }

    #[inline]
    fn align_bits(&mut self) -> () {
        let left = self.left() & 63;

        self.skip_bits(left);
    }
}

macro_rules! endian_reader {
    {$name: ident} => {
        #[derive(Debug, Clone, Copy)]
        pub struct $name<'a> {
            buffer : &'a[u8], /// read buffer, 8-bytes padded
            index : usize,
            cache : u64,
            left : usize,
        }
        impl <'a> BitReadInternal for $name<'a> {
            #[inline]
            fn left(&self) -> usize {
                self.left
            }
            #[inline]
            fn refill32(&mut self) -> () {
                if !self.can_refill() {
                    return;
                }
                let val = self.fill32();

                self.cache  = Self::merge_val(val, self.cache,
                                              self.left, 32 - self.left);
                self.index += 4;
                self.left  += 32;
            }
            #[inline]
            fn refill64(&mut self) -> () {
                if !self.can_refill() {
                    return;
                }

                self.cache  = self.fill64();
                self.index += 8;
                self.left   = 64;
            }
        }

        impl <'a> BitRead<'a> for $name<'a> {
            fn new(buffer: &'a[u8]) -> $name<'a> {
                let mut reader = $name {
                    buffer: buffer,
                    index: 0,
                    cache: 0,
                    left: 0
                };

                reader.refill64();
                return reader;
            }
            #[inline]
            fn consumed(&self) -> usize {
                self.index * 8 - self.left
            }

            #[inline]
            fn available(&self) -> usize {
                (self.buffer.len() - self.index) * 8 + self.left
            }

            #[inline]
            fn can_refill(&self) -> bool {
                self.index < self.buffer.len() - 8
            }

            #[inline]
            fn skip_bits(&mut self, mut n:usize) -> () {
                if n == 0 {
                    return;
                }
                if self.left <= n {
                    n -= self.left;
                    self.skip_rem(n);
                    if n > 64 {
                        let skip = n / 8;

                        n -= skip * 8;
                        self.index += skip;
                    }
                    self.refill64();
                }

                self.skip_rem(n);
            }

        }
    }
}

endian_reader!{ BitReadLE }

impl <'a> BitReadEndian for BitReadLE<'a> {
    #[inline]
    fn peek_val(&mut self, n:usize) -> u64 {
        let v = self.cache & ((1u64 << n) - 1);

        return v;
    }
    #[inline]
    fn skip_rem(&mut self, n:usize) -> () {
        self.cache = self.cache >> n;
        self.left -= n;
    }
    #[inline]
    fn merge_val(msp:u64, lsp:u64, msb:usize, _:usize) -> u64 {
        msp << msb | lsp
    }
    #[inline(always)]
    fn fill32(&self) -> u64 {
        get_u32l(&self.buffer[self.index..]) as u64
    }
    #[inline(always)]
    fn fill64(&self) -> u64 {
        get_u64l(&self.buffer[self.index..])
    }
}

endian_reader!{ BitReadBE }

impl <'a> BitReadEndian for BitReadBE<'a> {
    #[inline]
    fn peek_val(&mut self, n:usize) -> u64 {
        self.cache >> (64 - n)
    }
    #[inline]
    fn skip_rem(&mut self, n:usize) -> () {
        self.cache = self.cache << n;
        self.left -= n;
    }
    #[inline]
    fn merge_val(msp:u64, lsp:u64, _:usize, lsb:usize) -> u64 {
        msp | lsp << lsb
    }
    #[inline(always)]
    fn fill32(&self) -> u64 {
        get_u32b(&self.buffer[self.index..]) as u64
    }
    #[inline(always)]
    fn fill64(&self) -> u64 {
        get_u64b(&self.buffer[self.index..])
    }
}


#[cfg(test)]
mod test {
    pub const CHECKBOARD0101: [u8; 128] = [0b01010101; 128];
    pub const CHECKBOARD0011: [u8; 128] = [0b00110011; 128];

    mod le {
        use super::super::*;
        use super::*;

        #[test]
        fn get_bit() {
            let b = &CHECKBOARD0101;
            let mut reader = BitReadLE::new(b);

            assert!(reader.get_bit());
            assert!(!reader.get_bit());
        }

        #[test]
        fn get_bits_64() {
            let b = &CHECKBOARD0101;
            let mut reader = BitReadLE::new(b);

            assert!(reader.get_bits_64(1) == 1);
            assert!(reader.get_bits_64(2) == 2);
            assert!(reader.get_bits_64(4) == 10);
            assert!(reader.get_bits_64(1) == 0);
            assert!(reader.get_bits_64(8) == 85);
        }

        #[test]
        fn peek_bits_64() {
            let mut reader = BitReadLE {
                buffer: &CHECKBOARD0101,
                index: 0,
                cache: 0,
                left: 0
            };

            assert!(reader.peek_bits_64(1) == 1);
            assert!(reader.peek_bits_64(1) == 1);
            assert!(reader.peek_bits_64(2) == 1);
            assert!(reader.peek_bits_64(2) == 1);
        }

        #[test]
        fn get_bits_32() {
            let mut reader = BitReadLE {
                buffer: &CHECKBOARD0101,
                index: 0,
                cache: 0,
                left: 0
            };

            assert!(reader.get_bits_32(1) == 1);
            assert!(reader.get_bits_32(2) == 2);
            assert!(reader.get_bits_32(4) == 10);
            assert!(reader.get_bits_32(1) == 0);
            assert!(reader.get_bits_32(8) == 85);
        }

        #[test]
        fn peek_bits_32() {
            let b = &CHECKBOARD0101;
            let mut reader = BitReadLE::new(b);

            assert!(reader.peek_bits_32(1) == 1);
            assert!(reader.peek_bits_32(1) == 1);
            assert!(reader.peek_bits_32(2) == 1);
            assert!(reader.peek_bits_32(2) == 1);
        }

        #[test]
        fn skip_bits() {
            let b = &CHECKBOARD0101;
            let mut reader = BitReadLE::new(b);

            reader.skip_bits(0);
            assert!(reader.peek_bits_32(1) == 1);
            reader.skip_bits(2);
            assert!(reader.peek_bits_32(1) == 1);
            reader.skip_bits(2);
            assert!(reader.peek_bits_32(1) == 1);
        }

        #[test]
        fn align_bits() {
            let b = &CHECKBOARD0011;
            let mut reader = BitReadLE::new(b);

            reader.align_bits();
            assert!(reader.get_bits_64(3) == 3);
            reader.align_bits();
            assert!(reader.get_bits_64(4) == 3);
            reader.skip_bits(1);
            reader.align_bits();
            assert!(reader.get_bits_64(4) == 3);
        }
    }
    mod be {
        use super::super::*;
        use super::*;

        #[test]
        fn get_bit() {
            let b = &CHECKBOARD0101;
            let mut reader = BitReadBE::new(b);

            assert!(!reader.get_bit());
            assert!(reader.get_bit());
        }

        #[test]
        fn get_bits_64() {
            let b = &CHECKBOARD0101;
            let mut reader = BitReadBE::new(b);

            assert!(reader.get_bits_64(1) == 0);
            assert!(reader.get_bits_64(2) == 2);
            assert!(reader.get_bits_64(4) == 10);
            assert!(reader.get_bits_64(1) == 1);
            assert!(reader.get_bits_64(8) == 85);
        }

        #[test]
        fn peek_bits_64() {
            let b = &CHECKBOARD0101;
            let mut reader = BitReadBE::new(b);

            assert!(reader.peek_bits_64(1) == 0);
            assert!(reader.peek_bits_64(1) == 0);
            assert!(reader.peek_bits_64(2) == 1);
            assert!(reader.peek_bits_64(2) == 1);
        }

        #[test]
        fn get_bits_32() {
            let b = &CHECKBOARD0101;
            let mut reader = BitReadBE::new(b);

            assert!(reader.get_bits_32(1) == 0);
            assert!(reader.get_bits_32(2) == 2);
            assert!(reader.get_bits_32(4) == 10);
            assert!(reader.get_bits_32(1) == 1);
            assert!(reader.get_bits_32(8) == 85);
        }

        #[test]
        fn peek_bits_32() {
            let b = &CHECKBOARD0101;
            let mut reader = BitReadBE::new(b);

            assert!(reader.peek_bits_32(1) == 0);
            assert!(reader.peek_bits_32(1) == 0);
            assert!(reader.peek_bits_32(2) == 1);
            assert!(reader.peek_bits_32(2) == 1);
        }

        #[test]
        fn skip_bits() {
            let b = &CHECKBOARD0101;
            let mut reader = BitReadBE::new(b);

            reader.skip_bits(0);
            assert!(reader.peek_bits_32(1) == 0);
            reader.skip_bits(2);
            assert!(reader.peek_bits_32(1) == 0);
            reader.skip_bits(2);
            assert!(reader.peek_bits_32(1) == 0);
        }

        #[test]
        fn align_bits() {
            let b = &CHECKBOARD0011;
            let mut reader = BitReadBE::new(b);

            reader.align_bits();
            assert!(reader.get_bits_64(3) == 1);
            reader.align_bits();
            assert!(reader.get_bits_64(4) == 3);
            reader.skip_bits(1);
            reader.align_bits();
            assert!(reader.get_bits_64(4) == 3);
        }
    }
}
