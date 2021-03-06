#[doc = "Reader of register CRC32INIRES_LO"]
pub type R = crate::R<u16, super::CRC32INIRES_LO>;
#[doc = "Writer for register CRC32INIRES_LO"]
pub type W = crate::W<u16, super::CRC32INIRES_LO>;
#[doc = "Register CRC32INIRES_LO `reset()`'s with value 0"]
impl crate::ResetValue for super::CRC32INIRES_LO {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CRC32INIRES_LO`"]
pub type CRC32INIRES_LO_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CRC32INIRES_LO`"]
pub struct CRC32INIRES_LO_W<'a> {
    w: &'a mut W,
}
impl<'a> CRC32INIRES_LO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u16) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - CRC32 initialization and result, lower 16 bits"]
    #[inline(always)]
    pub fn crc32inires_lo(&self) -> CRC32INIRES_LO_R {
        CRC32INIRES_LO_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - CRC32 initialization and result, lower 16 bits"]
    #[inline(always)]
    pub fn crc32inires_lo(&mut self) -> CRC32INIRES_LO_W {
        CRC32INIRES_LO_W { w: self }
    }
}
