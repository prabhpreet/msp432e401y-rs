#[doc = "Register `ADDR3L` reader"]
pub struct R(crate::R<ADDR3L_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADDR3L_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADDR3L_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADDR3L_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADDR3L` writer"]
pub struct W(crate::W<ADDR3L_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADDR3L_SPEC>;
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
impl From<crate::W<ADDR3L_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADDR3L_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EMAC_ADDR3L_ADDRLO` reader - MAC Address3 \\[31:0\\]"]
pub type EMAC_ADDR3L_ADDRLO_R = crate::FieldReader<u32, u32>;
#[doc = "Field `EMAC_ADDR3L_ADDRLO` writer - MAC Address3 \\[31:0\\]"]
pub type EMAC_ADDR3L_ADDRLO_W<'a> = crate::FieldWriter<'a, u32, ADDR3L_SPEC, u32, u32, 32, 0>;
impl R {
    #[doc = "Bits 0:31 - MAC Address3 \\[31:0\\]"]
    #[inline(always)]
    pub fn emac_addr3l_addrlo(&self) -> EMAC_ADDR3L_ADDRLO_R {
        EMAC_ADDR3L_ADDRLO_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - MAC Address3 \\[31:0\\]"]
    #[inline(always)]
    pub fn emac_addr3l_addrlo(&mut self) -> EMAC_ADDR3L_ADDRLO_W {
        EMAC_ADDR3L_ADDRLO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ethernet MAC Address 3 Low\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [addr3l](index.html) module"]
pub struct ADDR3L_SPEC;
impl crate::RegisterSpec for ADDR3L_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [addr3l::R](R) reader structure"]
impl crate::Readable for ADDR3L_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [addr3l::W](W) writer structure"]
impl crate::Writable for ADDR3L_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADDR3L to value 0"]
impl crate::Resettable for ADDR3L_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
