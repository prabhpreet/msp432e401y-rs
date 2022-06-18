#[doc = "Register `CTL` reader"]
pub struct R(crate::R<CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTL` writer"]
pub struct W(crate::W<CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTL_SPEC>;
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
impl From<crate::W<CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `QEI_CTL_ENABLE` reader - Enable QEI"]
pub type QEI_CTL_ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `QEI_CTL_ENABLE` writer - Enable QEI"]
pub type QEI_CTL_ENABLE_W<'a> = crate::BitWriter<'a, u32, CTL_SPEC, bool, 0>;
#[doc = "Field `QEI_CTL_SWAP` reader - Swap Signals"]
pub type QEI_CTL_SWAP_R = crate::BitReader<bool>;
#[doc = "Field `QEI_CTL_SWAP` writer - Swap Signals"]
pub type QEI_CTL_SWAP_W<'a> = crate::BitWriter<'a, u32, CTL_SPEC, bool, 1>;
#[doc = "Field `QEI_CTL_SIGMODE` reader - Signal Mode"]
pub type QEI_CTL_SIGMODE_R = crate::BitReader<bool>;
#[doc = "Field `QEI_CTL_SIGMODE` writer - Signal Mode"]
pub type QEI_CTL_SIGMODE_W<'a> = crate::BitWriter<'a, u32, CTL_SPEC, bool, 2>;
#[doc = "Field `QEI_CTL_CAPMODE` reader - Capture Mode"]
pub type QEI_CTL_CAPMODE_R = crate::BitReader<bool>;
#[doc = "Field `QEI_CTL_CAPMODE` writer - Capture Mode"]
pub type QEI_CTL_CAPMODE_W<'a> = crate::BitWriter<'a, u32, CTL_SPEC, bool, 3>;
#[doc = "Field `QEI_CTL_RESMODE` reader - Reset Mode"]
pub type QEI_CTL_RESMODE_R = crate::BitReader<bool>;
#[doc = "Field `QEI_CTL_RESMODE` writer - Reset Mode"]
pub type QEI_CTL_RESMODE_W<'a> = crate::BitWriter<'a, u32, CTL_SPEC, bool, 4>;
#[doc = "Field `QEI_CTL_VELEN` reader - Capture Velocity"]
pub type QEI_CTL_VELEN_R = crate::BitReader<bool>;
#[doc = "Field `QEI_CTL_VELEN` writer - Capture Velocity"]
pub type QEI_CTL_VELEN_W<'a> = crate::BitWriter<'a, u32, CTL_SPEC, bool, 5>;
#[doc = "Predivide Velocity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum QEI_CTL_VELDIV_A {
    #[doc = "0: QEI clock /1"]
    QEI_CTL_VELDIV_1 = 0,
    #[doc = "1: QEI clock /2"]
    QEI_CTL_VELDIV_2 = 1,
    #[doc = "2: QEI clock /4"]
    QEI_CTL_VELDIV_4 = 2,
    #[doc = "3: QEI clock /8"]
    QEI_CTL_VELDIV_8 = 3,
    #[doc = "4: QEI clock /16"]
    QEI_CTL_VELDIV_16 = 4,
    #[doc = "5: QEI clock /32"]
    QEI_CTL_VELDIV_32 = 5,
    #[doc = "6: QEI clock /64"]
    QEI_CTL_VELDIV_64 = 6,
    #[doc = "7: QEI clock /128"]
    QEI_CTL_VELDIV_128 = 7,
}
impl From<QEI_CTL_VELDIV_A> for u8 {
    #[inline(always)]
    fn from(variant: QEI_CTL_VELDIV_A) -> Self {
        variant as _
    }
}
#[doc = "Field `QEI_CTL_VELDIV` reader - Predivide Velocity"]
pub type QEI_CTL_VELDIV_R = crate::FieldReader<u8, QEI_CTL_VELDIV_A>;
impl QEI_CTL_VELDIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> QEI_CTL_VELDIV_A {
        match self.bits {
            0 => QEI_CTL_VELDIV_A::QEI_CTL_VELDIV_1,
            1 => QEI_CTL_VELDIV_A::QEI_CTL_VELDIV_2,
            2 => QEI_CTL_VELDIV_A::QEI_CTL_VELDIV_4,
            3 => QEI_CTL_VELDIV_A::QEI_CTL_VELDIV_8,
            4 => QEI_CTL_VELDIV_A::QEI_CTL_VELDIV_16,
            5 => QEI_CTL_VELDIV_A::QEI_CTL_VELDIV_32,
            6 => QEI_CTL_VELDIV_A::QEI_CTL_VELDIV_64,
            7 => QEI_CTL_VELDIV_A::QEI_CTL_VELDIV_128,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `QEI_CTL_VELDIV_1`"]
    #[inline(always)]
    pub fn is_qei_ctl_veldiv_1(&self) -> bool {
        *self == QEI_CTL_VELDIV_A::QEI_CTL_VELDIV_1
    }
    #[doc = "Checks if the value of the field is `QEI_CTL_VELDIV_2`"]
    #[inline(always)]
    pub fn is_qei_ctl_veldiv_2(&self) -> bool {
        *self == QEI_CTL_VELDIV_A::QEI_CTL_VELDIV_2
    }
    #[doc = "Checks if the value of the field is `QEI_CTL_VELDIV_4`"]
    #[inline(always)]
    pub fn is_qei_ctl_veldiv_4(&self) -> bool {
        *self == QEI_CTL_VELDIV_A::QEI_CTL_VELDIV_4
    }
    #[doc = "Checks if the value of the field is `QEI_CTL_VELDIV_8`"]
    #[inline(always)]
    pub fn is_qei_ctl_veldiv_8(&self) -> bool {
        *self == QEI_CTL_VELDIV_A::QEI_CTL_VELDIV_8
    }
    #[doc = "Checks if the value of the field is `QEI_CTL_VELDIV_16`"]
    #[inline(always)]
    pub fn is_qei_ctl_veldiv_16(&self) -> bool {
        *self == QEI_CTL_VELDIV_A::QEI_CTL_VELDIV_16
    }
    #[doc = "Checks if the value of the field is `QEI_CTL_VELDIV_32`"]
    #[inline(always)]
    pub fn is_qei_ctl_veldiv_32(&self) -> bool {
        *self == QEI_CTL_VELDIV_A::QEI_CTL_VELDIV_32
    }
    #[doc = "Checks if the value of the field is `QEI_CTL_VELDIV_64`"]
    #[inline(always)]
    pub fn is_qei_ctl_veldiv_64(&self) -> bool {
        *self == QEI_CTL_VELDIV_A::QEI_CTL_VELDIV_64
    }
    #[doc = "Checks if the value of the field is `QEI_CTL_VELDIV_128`"]
    #[inline(always)]
    pub fn is_qei_ctl_veldiv_128(&self) -> bool {
        *self == QEI_CTL_VELDIV_A::QEI_CTL_VELDIV_128
    }
}
#[doc = "Field `QEI_CTL_VELDIV` writer - Predivide Velocity"]
pub type QEI_CTL_VELDIV_W<'a> =
    crate::FieldWriterSafe<'a, u32, CTL_SPEC, u8, QEI_CTL_VELDIV_A, 3, 6>;
impl<'a> QEI_CTL_VELDIV_W<'a> {
    #[doc = "QEI clock /1"]
    #[inline(always)]
    pub fn qei_ctl_veldiv_1(self) -> &'a mut W {
        self.variant(QEI_CTL_VELDIV_A::QEI_CTL_VELDIV_1)
    }
    #[doc = "QEI clock /2"]
    #[inline(always)]
    pub fn qei_ctl_veldiv_2(self) -> &'a mut W {
        self.variant(QEI_CTL_VELDIV_A::QEI_CTL_VELDIV_2)
    }
    #[doc = "QEI clock /4"]
    #[inline(always)]
    pub fn qei_ctl_veldiv_4(self) -> &'a mut W {
        self.variant(QEI_CTL_VELDIV_A::QEI_CTL_VELDIV_4)
    }
    #[doc = "QEI clock /8"]
    #[inline(always)]
    pub fn qei_ctl_veldiv_8(self) -> &'a mut W {
        self.variant(QEI_CTL_VELDIV_A::QEI_CTL_VELDIV_8)
    }
    #[doc = "QEI clock /16"]
    #[inline(always)]
    pub fn qei_ctl_veldiv_16(self) -> &'a mut W {
        self.variant(QEI_CTL_VELDIV_A::QEI_CTL_VELDIV_16)
    }
    #[doc = "QEI clock /32"]
    #[inline(always)]
    pub fn qei_ctl_veldiv_32(self) -> &'a mut W {
        self.variant(QEI_CTL_VELDIV_A::QEI_CTL_VELDIV_32)
    }
    #[doc = "QEI clock /64"]
    #[inline(always)]
    pub fn qei_ctl_veldiv_64(self) -> &'a mut W {
        self.variant(QEI_CTL_VELDIV_A::QEI_CTL_VELDIV_64)
    }
    #[doc = "QEI clock /128"]
    #[inline(always)]
    pub fn qei_ctl_veldiv_128(self) -> &'a mut W {
        self.variant(QEI_CTL_VELDIV_A::QEI_CTL_VELDIV_128)
    }
}
#[doc = "Field `QEI_CTL_INVA` reader - Invert PhA"]
pub type QEI_CTL_INVA_R = crate::BitReader<bool>;
#[doc = "Field `QEI_CTL_INVA` writer - Invert PhA"]
pub type QEI_CTL_INVA_W<'a> = crate::BitWriter<'a, u32, CTL_SPEC, bool, 9>;
#[doc = "Field `QEI_CTL_INVB` reader - Invert PhB"]
pub type QEI_CTL_INVB_R = crate::BitReader<bool>;
#[doc = "Field `QEI_CTL_INVB` writer - Invert PhB"]
pub type QEI_CTL_INVB_W<'a> = crate::BitWriter<'a, u32, CTL_SPEC, bool, 10>;
#[doc = "Field `QEI_CTL_INVI` reader - Invert Index Pulse"]
pub type QEI_CTL_INVI_R = crate::BitReader<bool>;
#[doc = "Field `QEI_CTL_INVI` writer - Invert Index Pulse"]
pub type QEI_CTL_INVI_W<'a> = crate::BitWriter<'a, u32, CTL_SPEC, bool, 11>;
#[doc = "Field `QEI_CTL_STALLEN` reader - Stall QEI"]
pub type QEI_CTL_STALLEN_R = crate::BitReader<bool>;
#[doc = "Field `QEI_CTL_STALLEN` writer - Stall QEI"]
pub type QEI_CTL_STALLEN_W<'a> = crate::BitWriter<'a, u32, CTL_SPEC, bool, 12>;
#[doc = "Field `QEI_CTL_FILTEN` reader - Enable Input Filter"]
pub type QEI_CTL_FILTEN_R = crate::BitReader<bool>;
#[doc = "Field `QEI_CTL_FILTEN` writer - Enable Input Filter"]
pub type QEI_CTL_FILTEN_W<'a> = crate::BitWriter<'a, u32, CTL_SPEC, bool, 13>;
#[doc = "Field `QEI_CTL_FILTCNT` reader - Input Filter Prescale Count"]
pub type QEI_CTL_FILTCNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `QEI_CTL_FILTCNT` writer - Input Filter Prescale Count"]
pub type QEI_CTL_FILTCNT_W<'a> = crate::FieldWriter<'a, u32, CTL_SPEC, u8, u8, 4, 16>;
impl R {
    #[doc = "Bit 0 - Enable QEI"]
    #[inline(always)]
    pub fn qei_ctl_enable(&self) -> QEI_CTL_ENABLE_R {
        QEI_CTL_ENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Swap Signals"]
    #[inline(always)]
    pub fn qei_ctl_swap(&self) -> QEI_CTL_SWAP_R {
        QEI_CTL_SWAP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Signal Mode"]
    #[inline(always)]
    pub fn qei_ctl_sigmode(&self) -> QEI_CTL_SIGMODE_R {
        QEI_CTL_SIGMODE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Capture Mode"]
    #[inline(always)]
    pub fn qei_ctl_capmode(&self) -> QEI_CTL_CAPMODE_R {
        QEI_CTL_CAPMODE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Reset Mode"]
    #[inline(always)]
    pub fn qei_ctl_resmode(&self) -> QEI_CTL_RESMODE_R {
        QEI_CTL_RESMODE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Capture Velocity"]
    #[inline(always)]
    pub fn qei_ctl_velen(&self) -> QEI_CTL_VELEN_R {
        QEI_CTL_VELEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:8 - Predivide Velocity"]
    #[inline(always)]
    pub fn qei_ctl_veldiv(&self) -> QEI_CTL_VELDIV_R {
        QEI_CTL_VELDIV_R::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bit 9 - Invert PhA"]
    #[inline(always)]
    pub fn qei_ctl_inva(&self) -> QEI_CTL_INVA_R {
        QEI_CTL_INVA_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Invert PhB"]
    #[inline(always)]
    pub fn qei_ctl_invb(&self) -> QEI_CTL_INVB_R {
        QEI_CTL_INVB_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Invert Index Pulse"]
    #[inline(always)]
    pub fn qei_ctl_invi(&self) -> QEI_CTL_INVI_R {
        QEI_CTL_INVI_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Stall QEI"]
    #[inline(always)]
    pub fn qei_ctl_stallen(&self) -> QEI_CTL_STALLEN_R {
        QEI_CTL_STALLEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enable Input Filter"]
    #[inline(always)]
    pub fn qei_ctl_filten(&self) -> QEI_CTL_FILTEN_R {
        QEI_CTL_FILTEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 16:19 - Input Filter Prescale Count"]
    #[inline(always)]
    pub fn qei_ctl_filtcnt(&self) -> QEI_CTL_FILTCNT_R {
        QEI_CTL_FILTCNT_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enable QEI"]
    #[inline(always)]
    pub fn qei_ctl_enable(&mut self) -> QEI_CTL_ENABLE_W {
        QEI_CTL_ENABLE_W::new(self)
    }
    #[doc = "Bit 1 - Swap Signals"]
    #[inline(always)]
    pub fn qei_ctl_swap(&mut self) -> QEI_CTL_SWAP_W {
        QEI_CTL_SWAP_W::new(self)
    }
    #[doc = "Bit 2 - Signal Mode"]
    #[inline(always)]
    pub fn qei_ctl_sigmode(&mut self) -> QEI_CTL_SIGMODE_W {
        QEI_CTL_SIGMODE_W::new(self)
    }
    #[doc = "Bit 3 - Capture Mode"]
    #[inline(always)]
    pub fn qei_ctl_capmode(&mut self) -> QEI_CTL_CAPMODE_W {
        QEI_CTL_CAPMODE_W::new(self)
    }
    #[doc = "Bit 4 - Reset Mode"]
    #[inline(always)]
    pub fn qei_ctl_resmode(&mut self) -> QEI_CTL_RESMODE_W {
        QEI_CTL_RESMODE_W::new(self)
    }
    #[doc = "Bit 5 - Capture Velocity"]
    #[inline(always)]
    pub fn qei_ctl_velen(&mut self) -> QEI_CTL_VELEN_W {
        QEI_CTL_VELEN_W::new(self)
    }
    #[doc = "Bits 6:8 - Predivide Velocity"]
    #[inline(always)]
    pub fn qei_ctl_veldiv(&mut self) -> QEI_CTL_VELDIV_W {
        QEI_CTL_VELDIV_W::new(self)
    }
    #[doc = "Bit 9 - Invert PhA"]
    #[inline(always)]
    pub fn qei_ctl_inva(&mut self) -> QEI_CTL_INVA_W {
        QEI_CTL_INVA_W::new(self)
    }
    #[doc = "Bit 10 - Invert PhB"]
    #[inline(always)]
    pub fn qei_ctl_invb(&mut self) -> QEI_CTL_INVB_W {
        QEI_CTL_INVB_W::new(self)
    }
    #[doc = "Bit 11 - Invert Index Pulse"]
    #[inline(always)]
    pub fn qei_ctl_invi(&mut self) -> QEI_CTL_INVI_W {
        QEI_CTL_INVI_W::new(self)
    }
    #[doc = "Bit 12 - Stall QEI"]
    #[inline(always)]
    pub fn qei_ctl_stallen(&mut self) -> QEI_CTL_STALLEN_W {
        QEI_CTL_STALLEN_W::new(self)
    }
    #[doc = "Bit 13 - Enable Input Filter"]
    #[inline(always)]
    pub fn qei_ctl_filten(&mut self) -> QEI_CTL_FILTEN_W {
        QEI_CTL_FILTEN_W::new(self)
    }
    #[doc = "Bits 16:19 - Input Filter Prescale Count"]
    #[inline(always)]
    pub fn qei_ctl_filtcnt(&mut self) -> QEI_CTL_FILTCNT_W {
        QEI_CTL_FILTCNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "QEI Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctl](index.html) module"]
pub struct CTL_SPEC;
impl crate::RegisterSpec for CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctl::R](R) reader structure"]
impl crate::Readable for CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctl::W](W) writer structure"]
impl crate::Writable for CTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTL to value 0"]
impl crate::Resettable for CTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
