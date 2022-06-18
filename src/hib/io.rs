#[doc = "Register `IO` reader"]
pub struct R(crate::R<IO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IO` writer"]
pub struct W(crate::W<IO_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IO_SPEC>;
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
impl From<crate::W<IO_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IO_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HIB_IO_WUUNLK` reader - I/O Wake Pad Configuration Enable"]
pub type HIB_IO_WUUNLK_R = crate::BitReader<bool>;
#[doc = "Field `HIB_IO_WUUNLK` writer - I/O Wake Pad Configuration Enable"]
pub type HIB_IO_WUUNLK_W<'a> = crate::BitWriter<'a, u32, IO_SPEC, bool, 0>;
#[doc = "Field `HIB_IO_WURSTEN` reader - Reset Wake Source Enable"]
pub type HIB_IO_WURSTEN_R = crate::BitReader<bool>;
#[doc = "Field `HIB_IO_WURSTEN` writer - Reset Wake Source Enable"]
pub type HIB_IO_WURSTEN_W<'a> = crate::BitWriter<'a, u32, IO_SPEC, bool, 4>;
#[doc = "Field `HIB_IO_IOWRC` reader - I/O Write Complete"]
pub type HIB_IO_IOWRC_R = crate::BitReader<bool>;
#[doc = "Field `HIB_IO_IOWRC` writer - I/O Write Complete"]
pub type HIB_IO_IOWRC_W<'a> = crate::BitWriter<'a, u32, IO_SPEC, bool, 31>;
impl R {
    #[doc = "Bit 0 - I/O Wake Pad Configuration Enable"]
    #[inline(always)]
    pub fn hib_io_wuunlk(&self) -> HIB_IO_WUUNLK_R {
        HIB_IO_WUUNLK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - Reset Wake Source Enable"]
    #[inline(always)]
    pub fn hib_io_wursten(&self) -> HIB_IO_WURSTEN_R {
        HIB_IO_WURSTEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 31 - I/O Write Complete"]
    #[inline(always)]
    pub fn hib_io_iowrc(&self) -> HIB_IO_IOWRC_R {
        HIB_IO_IOWRC_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - I/O Wake Pad Configuration Enable"]
    #[inline(always)]
    pub fn hib_io_wuunlk(&mut self) -> HIB_IO_WUUNLK_W {
        HIB_IO_WUUNLK_W::new(self)
    }
    #[doc = "Bit 4 - Reset Wake Source Enable"]
    #[inline(always)]
    pub fn hib_io_wursten(&mut self) -> HIB_IO_WURSTEN_W {
        HIB_IO_WURSTEN_W::new(self)
    }
    #[doc = "Bit 31 - I/O Write Complete"]
    #[inline(always)]
    pub fn hib_io_iowrc(&mut self) -> HIB_IO_IOWRC_W {
        HIB_IO_IOWRC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Hibernation IO Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [io](index.html) module"]
pub struct IO_SPEC;
impl crate::RegisterSpec for IO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [io::R](R) reader structure"]
impl crate::Readable for IO_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [io::W](W) writer structure"]
impl crate::Writable for IO_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IO to value 0"]
impl crate::Resettable for IO_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
