#[doc = "Register `DCGCGPIO` reader"]
pub struct R(crate::R<DCGCGPIO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DCGCGPIO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DCGCGPIO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DCGCGPIO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DCGCGPIO` writer"]
pub struct W(crate::W<DCGCGPIO_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DCGCGPIO_SPEC>;
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
impl From<crate::W<DCGCGPIO_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DCGCGPIO_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYSCTL_DCGCGPIO_D0` reader - GPIO Port A Deep-Sleep Mode Clock Gating Control"]
pub type SYSCTL_DCGCGPIO_D0_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_DCGCGPIO_D0` writer - GPIO Port A Deep-Sleep Mode Clock Gating Control"]
pub type SYSCTL_DCGCGPIO_D0_W<'a> = crate::BitWriter<'a, u32, DCGCGPIO_SPEC, bool, 0>;
#[doc = "Field `SYSCTL_DCGCGPIO_D1` reader - GPIO Port B Deep-Sleep Mode Clock Gating Control"]
pub type SYSCTL_DCGCGPIO_D1_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_DCGCGPIO_D1` writer - GPIO Port B Deep-Sleep Mode Clock Gating Control"]
pub type SYSCTL_DCGCGPIO_D1_W<'a> = crate::BitWriter<'a, u32, DCGCGPIO_SPEC, bool, 1>;
#[doc = "Field `SYSCTL_DCGCGPIO_D2` reader - GPIO Port C Deep-Sleep Mode Clock Gating Control"]
pub type SYSCTL_DCGCGPIO_D2_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_DCGCGPIO_D2` writer - GPIO Port C Deep-Sleep Mode Clock Gating Control"]
pub type SYSCTL_DCGCGPIO_D2_W<'a> = crate::BitWriter<'a, u32, DCGCGPIO_SPEC, bool, 2>;
#[doc = "Field `SYSCTL_DCGCGPIO_D3` reader - GPIO Port D Deep-Sleep Mode Clock Gating Control"]
pub type SYSCTL_DCGCGPIO_D3_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_DCGCGPIO_D3` writer - GPIO Port D Deep-Sleep Mode Clock Gating Control"]
pub type SYSCTL_DCGCGPIO_D3_W<'a> = crate::BitWriter<'a, u32, DCGCGPIO_SPEC, bool, 3>;
#[doc = "Field `SYSCTL_DCGCGPIO_D4` reader - GPIO Port E Deep-Sleep Mode Clock Gating Control"]
pub type SYSCTL_DCGCGPIO_D4_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_DCGCGPIO_D4` writer - GPIO Port E Deep-Sleep Mode Clock Gating Control"]
pub type SYSCTL_DCGCGPIO_D4_W<'a> = crate::BitWriter<'a, u32, DCGCGPIO_SPEC, bool, 4>;
#[doc = "Field `SYSCTL_DCGCGPIO_D5` reader - GPIO Port F Deep-Sleep Mode Clock Gating Control"]
pub type SYSCTL_DCGCGPIO_D5_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_DCGCGPIO_D5` writer - GPIO Port F Deep-Sleep Mode Clock Gating Control"]
pub type SYSCTL_DCGCGPIO_D5_W<'a> = crate::BitWriter<'a, u32, DCGCGPIO_SPEC, bool, 5>;
#[doc = "Field `SYSCTL_DCGCGPIO_D6` reader - GPIO Port G Deep-Sleep Mode Clock Gating Control"]
pub type SYSCTL_DCGCGPIO_D6_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_DCGCGPIO_D6` writer - GPIO Port G Deep-Sleep Mode Clock Gating Control"]
pub type SYSCTL_DCGCGPIO_D6_W<'a> = crate::BitWriter<'a, u32, DCGCGPIO_SPEC, bool, 6>;
#[doc = "Field `SYSCTL_DCGCGPIO_D7` reader - GPIO Port H Deep-Sleep Mode Clock Gating Control"]
pub type SYSCTL_DCGCGPIO_D7_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_DCGCGPIO_D7` writer - GPIO Port H Deep-Sleep Mode Clock Gating Control"]
pub type SYSCTL_DCGCGPIO_D7_W<'a> = crate::BitWriter<'a, u32, DCGCGPIO_SPEC, bool, 7>;
#[doc = "Field `SYSCTL_DCGCGPIO_D8` reader - GPIO Port J Deep-Sleep Mode Clock Gating Control"]
pub type SYSCTL_DCGCGPIO_D8_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_DCGCGPIO_D8` writer - GPIO Port J Deep-Sleep Mode Clock Gating Control"]
pub type SYSCTL_DCGCGPIO_D8_W<'a> = crate::BitWriter<'a, u32, DCGCGPIO_SPEC, bool, 8>;
#[doc = "Field `SYSCTL_DCGCGPIO_D9` reader - GPIO Port K Deep-Sleep Mode Clock Gating Control"]
pub type SYSCTL_DCGCGPIO_D9_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_DCGCGPIO_D9` writer - GPIO Port K Deep-Sleep Mode Clock Gating Control"]
pub type SYSCTL_DCGCGPIO_D9_W<'a> = crate::BitWriter<'a, u32, DCGCGPIO_SPEC, bool, 9>;
#[doc = "Field `SYSCTL_DCGCGPIO_D10` reader - GPIO Port L Deep-Sleep Mode Clock Gating Control"]
pub type SYSCTL_DCGCGPIO_D10_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_DCGCGPIO_D10` writer - GPIO Port L Deep-Sleep Mode Clock Gating Control"]
pub type SYSCTL_DCGCGPIO_D10_W<'a> = crate::BitWriter<'a, u32, DCGCGPIO_SPEC, bool, 10>;
#[doc = "Field `SYSCTL_DCGCGPIO_D11` reader - GPIO Port M Deep-Sleep Mode Clock Gating Control"]
pub type SYSCTL_DCGCGPIO_D11_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_DCGCGPIO_D11` writer - GPIO Port M Deep-Sleep Mode Clock Gating Control"]
pub type SYSCTL_DCGCGPIO_D11_W<'a> = crate::BitWriter<'a, u32, DCGCGPIO_SPEC, bool, 11>;
#[doc = "Field `SYSCTL_DCGCGPIO_D12` reader - GPIO Port N Deep-Sleep Mode Clock Gating Control"]
pub type SYSCTL_DCGCGPIO_D12_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_DCGCGPIO_D12` writer - GPIO Port N Deep-Sleep Mode Clock Gating Control"]
pub type SYSCTL_DCGCGPIO_D12_W<'a> = crate::BitWriter<'a, u32, DCGCGPIO_SPEC, bool, 12>;
#[doc = "Field `SYSCTL_DCGCGPIO_D13` reader - GPIO Port P Deep-Sleep Mode Clock Gating Control"]
pub type SYSCTL_DCGCGPIO_D13_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_DCGCGPIO_D13` writer - GPIO Port P Deep-Sleep Mode Clock Gating Control"]
pub type SYSCTL_DCGCGPIO_D13_W<'a> = crate::BitWriter<'a, u32, DCGCGPIO_SPEC, bool, 13>;
#[doc = "Field `SYSCTL_DCGCGPIO_D14` reader - GPIO Port Q Deep-Sleep Mode Clock Gating Control"]
pub type SYSCTL_DCGCGPIO_D14_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_DCGCGPIO_D14` writer - GPIO Port Q Deep-Sleep Mode Clock Gating Control"]
pub type SYSCTL_DCGCGPIO_D14_W<'a> = crate::BitWriter<'a, u32, DCGCGPIO_SPEC, bool, 14>;
impl R {
    #[doc = "Bit 0 - GPIO Port A Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgcgpio_d0(&self) -> SYSCTL_DCGCGPIO_D0_R {
        SYSCTL_DCGCGPIO_D0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - GPIO Port B Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgcgpio_d1(&self) -> SYSCTL_DCGCGPIO_D1_R {
        SYSCTL_DCGCGPIO_D1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - GPIO Port C Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgcgpio_d2(&self) -> SYSCTL_DCGCGPIO_D2_R {
        SYSCTL_DCGCGPIO_D2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - GPIO Port D Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgcgpio_d3(&self) -> SYSCTL_DCGCGPIO_D3_R {
        SYSCTL_DCGCGPIO_D3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - GPIO Port E Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgcgpio_d4(&self) -> SYSCTL_DCGCGPIO_D4_R {
        SYSCTL_DCGCGPIO_D4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - GPIO Port F Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgcgpio_d5(&self) -> SYSCTL_DCGCGPIO_D5_R {
        SYSCTL_DCGCGPIO_D5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - GPIO Port G Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgcgpio_d6(&self) -> SYSCTL_DCGCGPIO_D6_R {
        SYSCTL_DCGCGPIO_D6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - GPIO Port H Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgcgpio_d7(&self) -> SYSCTL_DCGCGPIO_D7_R {
        SYSCTL_DCGCGPIO_D7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - GPIO Port J Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgcgpio_d8(&self) -> SYSCTL_DCGCGPIO_D8_R {
        SYSCTL_DCGCGPIO_D8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - GPIO Port K Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgcgpio_d9(&self) -> SYSCTL_DCGCGPIO_D9_R {
        SYSCTL_DCGCGPIO_D9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - GPIO Port L Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgcgpio_d10(&self) -> SYSCTL_DCGCGPIO_D10_R {
        SYSCTL_DCGCGPIO_D10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - GPIO Port M Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgcgpio_d11(&self) -> SYSCTL_DCGCGPIO_D11_R {
        SYSCTL_DCGCGPIO_D11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - GPIO Port N Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgcgpio_d12(&self) -> SYSCTL_DCGCGPIO_D12_R {
        SYSCTL_DCGCGPIO_D12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - GPIO Port P Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgcgpio_d13(&self) -> SYSCTL_DCGCGPIO_D13_R {
        SYSCTL_DCGCGPIO_D13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - GPIO Port Q Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgcgpio_d14(&self) -> SYSCTL_DCGCGPIO_D14_R {
        SYSCTL_DCGCGPIO_D14_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - GPIO Port A Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgcgpio_d0(&mut self) -> SYSCTL_DCGCGPIO_D0_W {
        SYSCTL_DCGCGPIO_D0_W::new(self)
    }
    #[doc = "Bit 1 - GPIO Port B Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgcgpio_d1(&mut self) -> SYSCTL_DCGCGPIO_D1_W {
        SYSCTL_DCGCGPIO_D1_W::new(self)
    }
    #[doc = "Bit 2 - GPIO Port C Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgcgpio_d2(&mut self) -> SYSCTL_DCGCGPIO_D2_W {
        SYSCTL_DCGCGPIO_D2_W::new(self)
    }
    #[doc = "Bit 3 - GPIO Port D Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgcgpio_d3(&mut self) -> SYSCTL_DCGCGPIO_D3_W {
        SYSCTL_DCGCGPIO_D3_W::new(self)
    }
    #[doc = "Bit 4 - GPIO Port E Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgcgpio_d4(&mut self) -> SYSCTL_DCGCGPIO_D4_W {
        SYSCTL_DCGCGPIO_D4_W::new(self)
    }
    #[doc = "Bit 5 - GPIO Port F Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgcgpio_d5(&mut self) -> SYSCTL_DCGCGPIO_D5_W {
        SYSCTL_DCGCGPIO_D5_W::new(self)
    }
    #[doc = "Bit 6 - GPIO Port G Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgcgpio_d6(&mut self) -> SYSCTL_DCGCGPIO_D6_W {
        SYSCTL_DCGCGPIO_D6_W::new(self)
    }
    #[doc = "Bit 7 - GPIO Port H Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgcgpio_d7(&mut self) -> SYSCTL_DCGCGPIO_D7_W {
        SYSCTL_DCGCGPIO_D7_W::new(self)
    }
    #[doc = "Bit 8 - GPIO Port J Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgcgpio_d8(&mut self) -> SYSCTL_DCGCGPIO_D8_W {
        SYSCTL_DCGCGPIO_D8_W::new(self)
    }
    #[doc = "Bit 9 - GPIO Port K Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgcgpio_d9(&mut self) -> SYSCTL_DCGCGPIO_D9_W {
        SYSCTL_DCGCGPIO_D9_W::new(self)
    }
    #[doc = "Bit 10 - GPIO Port L Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgcgpio_d10(&mut self) -> SYSCTL_DCGCGPIO_D10_W {
        SYSCTL_DCGCGPIO_D10_W::new(self)
    }
    #[doc = "Bit 11 - GPIO Port M Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgcgpio_d11(&mut self) -> SYSCTL_DCGCGPIO_D11_W {
        SYSCTL_DCGCGPIO_D11_W::new(self)
    }
    #[doc = "Bit 12 - GPIO Port N Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgcgpio_d12(&mut self) -> SYSCTL_DCGCGPIO_D12_W {
        SYSCTL_DCGCGPIO_D12_W::new(self)
    }
    #[doc = "Bit 13 - GPIO Port P Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgcgpio_d13(&mut self) -> SYSCTL_DCGCGPIO_D13_W {
        SYSCTL_DCGCGPIO_D13_W::new(self)
    }
    #[doc = "Bit 14 - GPIO Port Q Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgcgpio_d14(&mut self) -> SYSCTL_DCGCGPIO_D14_W {
        SYSCTL_DCGCGPIO_D14_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "General-Purpose Input/Output Deep-Sleep Mode Clock Gating Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcgcgpio](index.html) module"]
pub struct DCGCGPIO_SPEC;
impl crate::RegisterSpec for DCGCGPIO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dcgcgpio::R](R) reader structure"]
impl crate::Readable for DCGCGPIO_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dcgcgpio::W](W) writer structure"]
impl crate::Writable for DCGCGPIO_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DCGCGPIO to value 0"]
impl crate::Resettable for DCGCGPIO_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
