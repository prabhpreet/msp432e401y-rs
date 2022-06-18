#[doc = "Register `FIFOLVL` reader"]
pub struct R(crate::R<FIFOLVL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FIFOLVL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FIFOLVL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FIFOLVL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FIFOLVL` writer"]
pub struct W(crate::W<FIFOLVL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FIFOLVL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<FIFOLVL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FIFOLVL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Read FIFO\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EPI_FIFOLVL_RDFIFO_A {
    #[doc = "1: Trigger when there are 1 or more entries in the NBRFIFO"]
    EPI_FIFOLVL_RDFIFO_1 = 1,
    #[doc = "2: Trigger when there are 2 or more entries in the NBRFIFO"]
    EPI_FIFOLVL_RDFIFO_2 = 2,
    #[doc = "3: Trigger when there are 4 or more entries in the NBRFIFO"]
    EPI_FIFOLVL_RDFIFO_4 = 3,
    #[doc = "4: Trigger when there are 6 or more entries in the NBRFIFO"]
    EPI_FIFOLVL_RDFIFO_6 = 4,
    #[doc = "5: Trigger when there are 7 or more entries in the NBRFIFO"]
    EPI_FIFOLVL_RDFIFO_7 = 5,
    #[doc = "6: Trigger when there are 8 entries in the NBRFIFO"]
    EPI_FIFOLVL_RDFIFO_8 = 6,
}
impl From<EPI_FIFOLVL_RDFIFO_A> for u8 {
    #[inline(always)]
    fn from(variant: EPI_FIFOLVL_RDFIFO_A) -> Self {
        variant as _
    }
}
#[doc = "Field `EPI_FIFOLVL_RDFIFO` reader - Read FIFO"]
pub type EPI_FIFOLVL_RDFIFO_R = crate::FieldReader<u8, EPI_FIFOLVL_RDFIFO_A>;
impl EPI_FIFOLVL_RDFIFO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EPI_FIFOLVL_RDFIFO_A> {
        match self.bits {
            1 => Some(EPI_FIFOLVL_RDFIFO_A::EPI_FIFOLVL_RDFIFO_1),
            2 => Some(EPI_FIFOLVL_RDFIFO_A::EPI_FIFOLVL_RDFIFO_2),
            3 => Some(EPI_FIFOLVL_RDFIFO_A::EPI_FIFOLVL_RDFIFO_4),
            4 => Some(EPI_FIFOLVL_RDFIFO_A::EPI_FIFOLVL_RDFIFO_6),
            5 => Some(EPI_FIFOLVL_RDFIFO_A::EPI_FIFOLVL_RDFIFO_7),
            6 => Some(EPI_FIFOLVL_RDFIFO_A::EPI_FIFOLVL_RDFIFO_8),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `EPI_FIFOLVL_RDFIFO_1`"]
    #[inline(always)]
    pub fn is_epi_fifolvl_rdfifo_1(&self) -> bool {
        *self == EPI_FIFOLVL_RDFIFO_A::EPI_FIFOLVL_RDFIFO_1
    }
    #[doc = "Checks if the value of the field is `EPI_FIFOLVL_RDFIFO_2`"]
    #[inline(always)]
    pub fn is_epi_fifolvl_rdfifo_2(&self) -> bool {
        *self == EPI_FIFOLVL_RDFIFO_A::EPI_FIFOLVL_RDFIFO_2
    }
    #[doc = "Checks if the value of the field is `EPI_FIFOLVL_RDFIFO_4`"]
    #[inline(always)]
    pub fn is_epi_fifolvl_rdfifo_4(&self) -> bool {
        *self == EPI_FIFOLVL_RDFIFO_A::EPI_FIFOLVL_RDFIFO_4
    }
    #[doc = "Checks if the value of the field is `EPI_FIFOLVL_RDFIFO_6`"]
    #[inline(always)]
    pub fn is_epi_fifolvl_rdfifo_6(&self) -> bool {
        *self == EPI_FIFOLVL_RDFIFO_A::EPI_FIFOLVL_RDFIFO_6
    }
    #[doc = "Checks if the value of the field is `EPI_FIFOLVL_RDFIFO_7`"]
    #[inline(always)]
    pub fn is_epi_fifolvl_rdfifo_7(&self) -> bool {
        *self == EPI_FIFOLVL_RDFIFO_A::EPI_FIFOLVL_RDFIFO_7
    }
    #[doc = "Checks if the value of the field is `EPI_FIFOLVL_RDFIFO_8`"]
    #[inline(always)]
    pub fn is_epi_fifolvl_rdfifo_8(&self) -> bool {
        *self == EPI_FIFOLVL_RDFIFO_A::EPI_FIFOLVL_RDFIFO_8
    }
}
#[doc = "Field `EPI_FIFOLVL_RDFIFO` writer - Read FIFO"]
pub type EPI_FIFOLVL_RDFIFO_W<'a> =
    crate::FieldWriter<'a, u32, FIFOLVL_SPEC, u8, EPI_FIFOLVL_RDFIFO_A, 3, 0>;
impl<'a> EPI_FIFOLVL_RDFIFO_W<'a> {
    #[doc = "Trigger when there are 1 or more entries in the NBRFIFO"]
    #[inline(always)]
    pub fn epi_fifolvl_rdfifo_1(self) -> &'a mut W {
        self.variant(EPI_FIFOLVL_RDFIFO_A::EPI_FIFOLVL_RDFIFO_1)
    }
    #[doc = "Trigger when there are 2 or more entries in the NBRFIFO"]
    #[inline(always)]
    pub fn epi_fifolvl_rdfifo_2(self) -> &'a mut W {
        self.variant(EPI_FIFOLVL_RDFIFO_A::EPI_FIFOLVL_RDFIFO_2)
    }
    #[doc = "Trigger when there are 4 or more entries in the NBRFIFO"]
    #[inline(always)]
    pub fn epi_fifolvl_rdfifo_4(self) -> &'a mut W {
        self.variant(EPI_FIFOLVL_RDFIFO_A::EPI_FIFOLVL_RDFIFO_4)
    }
    #[doc = "Trigger when there are 6 or more entries in the NBRFIFO"]
    #[inline(always)]
    pub fn epi_fifolvl_rdfifo_6(self) -> &'a mut W {
        self.variant(EPI_FIFOLVL_RDFIFO_A::EPI_FIFOLVL_RDFIFO_6)
    }
    #[doc = "Trigger when there are 7 or more entries in the NBRFIFO"]
    #[inline(always)]
    pub fn epi_fifolvl_rdfifo_7(self) -> &'a mut W {
        self.variant(EPI_FIFOLVL_RDFIFO_A::EPI_FIFOLVL_RDFIFO_7)
    }
    #[doc = "Trigger when there are 8 entries in the NBRFIFO"]
    #[inline(always)]
    pub fn epi_fifolvl_rdfifo_8(self) -> &'a mut W {
        self.variant(EPI_FIFOLVL_RDFIFO_A::EPI_FIFOLVL_RDFIFO_8)
    }
}
#[doc = "Write FIFO\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EPI_FIFOLVL_WRFIFO_A {
    #[doc = "0: Interrupt is triggered while WRFIFO is empty."]
    EPI_FIFOLVL_WRFIFO_EMPT = 0,
    #[doc = "2: Interrupt is triggered until there are only two slots available. Thus, trigger is deasserted when there are two WRFIFO entries present. This configuration is optimized for bursts of 2"]
    EPI_FIFOLVL_WRFIFO_2 = 2,
    #[doc = "3: Interrupt is triggered until there is one WRFIFO entry available. This configuration expects only single writes"]
    EPI_FIFOLVL_WRFIFO_1 = 3,
    #[doc = "4: Trigger interrupt when WRFIFO is not full, meaning trigger will continue to assert until there are four entries in the WRFIFO"]
    EPI_FIFOLVL_WRFIFO_NFULL = 4,
}
impl From<EPI_FIFOLVL_WRFIFO_A> for u8 {
    #[inline(always)]
    fn from(variant: EPI_FIFOLVL_WRFIFO_A) -> Self {
        variant as _
    }
}
#[doc = "Field `EPI_FIFOLVL_WRFIFO` reader - Write FIFO"]
pub type EPI_FIFOLVL_WRFIFO_R = crate::FieldReader<u8, EPI_FIFOLVL_WRFIFO_A>;
impl EPI_FIFOLVL_WRFIFO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EPI_FIFOLVL_WRFIFO_A> {
        match self.bits {
            0 => Some(EPI_FIFOLVL_WRFIFO_A::EPI_FIFOLVL_WRFIFO_EMPT),
            2 => Some(EPI_FIFOLVL_WRFIFO_A::EPI_FIFOLVL_WRFIFO_2),
            3 => Some(EPI_FIFOLVL_WRFIFO_A::EPI_FIFOLVL_WRFIFO_1),
            4 => Some(EPI_FIFOLVL_WRFIFO_A::EPI_FIFOLVL_WRFIFO_NFULL),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `EPI_FIFOLVL_WRFIFO_EMPT`"]
    #[inline(always)]
    pub fn is_epi_fifolvl_wrfifo_empt(&self) -> bool {
        *self == EPI_FIFOLVL_WRFIFO_A::EPI_FIFOLVL_WRFIFO_EMPT
    }
    #[doc = "Checks if the value of the field is `EPI_FIFOLVL_WRFIFO_2`"]
    #[inline(always)]
    pub fn is_epi_fifolvl_wrfifo_2(&self) -> bool {
        *self == EPI_FIFOLVL_WRFIFO_A::EPI_FIFOLVL_WRFIFO_2
    }
    #[doc = "Checks if the value of the field is `EPI_FIFOLVL_WRFIFO_1`"]
    #[inline(always)]
    pub fn is_epi_fifolvl_wrfifo_1(&self) -> bool {
        *self == EPI_FIFOLVL_WRFIFO_A::EPI_FIFOLVL_WRFIFO_1
    }
    #[doc = "Checks if the value of the field is `EPI_FIFOLVL_WRFIFO_NFULL`"]
    #[inline(always)]
    pub fn is_epi_fifolvl_wrfifo_nfull(&self) -> bool {
        *self == EPI_FIFOLVL_WRFIFO_A::EPI_FIFOLVL_WRFIFO_NFULL
    }
}
#[doc = "Field `EPI_FIFOLVL_WRFIFO` writer - Write FIFO"]
pub type EPI_FIFOLVL_WRFIFO_W<'a> =
    crate::FieldWriter<'a, u32, FIFOLVL_SPEC, u8, EPI_FIFOLVL_WRFIFO_A, 3, 4>;
impl<'a> EPI_FIFOLVL_WRFIFO_W<'a> {
    #[doc = "Interrupt is triggered while WRFIFO is empty."]
    #[inline(always)]
    pub fn epi_fifolvl_wrfifo_empt(self) -> &'a mut W {
        self.variant(EPI_FIFOLVL_WRFIFO_A::EPI_FIFOLVL_WRFIFO_EMPT)
    }
    #[doc = "Interrupt is triggered until there are only two slots available. Thus, trigger is deasserted when there are two WRFIFO entries present. This configuration is optimized for bursts of 2"]
    #[inline(always)]
    pub fn epi_fifolvl_wrfifo_2(self) -> &'a mut W {
        self.variant(EPI_FIFOLVL_WRFIFO_A::EPI_FIFOLVL_WRFIFO_2)
    }
    #[doc = "Interrupt is triggered until there is one WRFIFO entry available. This configuration expects only single writes"]
    #[inline(always)]
    pub fn epi_fifolvl_wrfifo_1(self) -> &'a mut W {
        self.variant(EPI_FIFOLVL_WRFIFO_A::EPI_FIFOLVL_WRFIFO_1)
    }
    #[doc = "Trigger interrupt when WRFIFO is not full, meaning trigger will continue to assert until there are four entries in the WRFIFO"]
    #[inline(always)]
    pub fn epi_fifolvl_wrfifo_nfull(self) -> &'a mut W {
        self.variant(EPI_FIFOLVL_WRFIFO_A::EPI_FIFOLVL_WRFIFO_NFULL)
    }
}
#[doc = "Field `EPI_FIFOLVL_RSERR` reader - Read Stall Error"]
pub type EPI_FIFOLVL_RSERR_R = crate::BitReader<bool>;
#[doc = "Field `EPI_FIFOLVL_RSERR` writer - Read Stall Error"]
pub type EPI_FIFOLVL_RSERR_W<'a> = crate::BitWriter<'a, u32, FIFOLVL_SPEC, bool, 16>;
#[doc = "Field `EPI_FIFOLVL_WFERR` reader - Write Full Error"]
pub type EPI_FIFOLVL_WFERR_R = crate::BitReader<bool>;
#[doc = "Field `EPI_FIFOLVL_WFERR` writer - Write Full Error"]
pub type EPI_FIFOLVL_WFERR_W<'a> = crate::BitWriter<'a, u32, FIFOLVL_SPEC, bool, 17>;
impl R {
    #[doc = "Bits 0:2 - Read FIFO"]
    #[inline(always)]
    pub fn epi_fifolvl_rdfifo(&self) -> EPI_FIFOLVL_RDFIFO_R {
        EPI_FIFOLVL_RDFIFO_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - Write FIFO"]
    #[inline(always)]
    pub fn epi_fifolvl_wrfifo(&self) -> EPI_FIFOLVL_WRFIFO_R {
        EPI_FIFOLVL_WRFIFO_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 16 - Read Stall Error"]
    #[inline(always)]
    pub fn epi_fifolvl_rserr(&self) -> EPI_FIFOLVL_RSERR_R {
        EPI_FIFOLVL_RSERR_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Write Full Error"]
    #[inline(always)]
    pub fn epi_fifolvl_wferr(&self) -> EPI_FIFOLVL_WFERR_R {
        EPI_FIFOLVL_WFERR_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Read FIFO"]
    #[inline(always)]
    pub fn epi_fifolvl_rdfifo(&mut self) -> EPI_FIFOLVL_RDFIFO_W {
        EPI_FIFOLVL_RDFIFO_W::new(self)
    }
    #[doc = "Bits 4:6 - Write FIFO"]
    #[inline(always)]
    pub fn epi_fifolvl_wrfifo(&mut self) -> EPI_FIFOLVL_WRFIFO_W {
        EPI_FIFOLVL_WRFIFO_W::new(self)
    }
    #[doc = "Bit 16 - Read Stall Error"]
    #[inline(always)]
    pub fn epi_fifolvl_rserr(&mut self) -> EPI_FIFOLVL_RSERR_W {
        EPI_FIFOLVL_RSERR_W::new(self)
    }
    #[doc = "Bit 17 - Write Full Error"]
    #[inline(always)]
    pub fn epi_fifolvl_wferr(&mut self) -> EPI_FIFOLVL_WFERR_W {
        EPI_FIFOLVL_WFERR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EPI FIFO Level Selects\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifolvl](index.html) module"]
pub struct FIFOLVL_SPEC;
impl crate::RegisterSpec for FIFOLVL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fifolvl::R](R) reader structure"]
impl crate::Readable for FIFOLVL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fifolvl::W](W) writer structure"]
impl crate::Writable for FIFOLVL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FIFOLVL to value 0"]
impl crate::Resettable for FIFOLVL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
