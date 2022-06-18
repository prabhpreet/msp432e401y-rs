#[doc = "Register `KEY3_L` reader"]
pub struct R(crate::R<KEY3_L_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<KEY3_L_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<KEY3_L_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<KEY3_L_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `KEY3_L` writer"]
pub struct W(crate::W<KEY3_L_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<KEY3_L_SPEC>;
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
impl From<crate::W<KEY3_L_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<KEY3_L_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DES_KEY3_L_KEY` reader - Key Data"]
pub type DES_KEY3_L_KEY_R = crate::FieldReader<u32, u32>;
#[doc = "Field `DES_KEY3_L_KEY` writer - Key Data"]
pub type DES_KEY3_L_KEY_W<'a> = crate::FieldWriter<'a, u32, KEY3_L_SPEC, u32, u32, 32, 0>;
impl R {
    #[doc = "Bits 0:31 - Key Data"]
    #[inline(always)]
    pub fn des_key3_l_key(&self) -> DES_KEY3_L_KEY_R {
        DES_KEY3_L_KEY_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Key Data"]
    #[inline(always)]
    pub fn des_key3_l_key(&mut self) -> DES_KEY3_L_KEY_W {
        DES_KEY3_L_KEY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DES Key 3 LSW for 192-Bit Key\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [key3_l](index.html) module"]
pub struct KEY3_L_SPEC;
impl crate::RegisterSpec for KEY3_L_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [key3_l::R](R) reader structure"]
impl crate::Readable for KEY3_L_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [key3_l::W](W) writer structure"]
impl crate::Writable for KEY3_L_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets KEY3_L to value 0"]
impl crate::Resettable for KEY3_L_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
