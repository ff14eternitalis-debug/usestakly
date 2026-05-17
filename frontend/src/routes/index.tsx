import {
  HeroSection,
  LandingCta,
  LandingLivePreview,
  LandingPrinciples
} from "../features/landing";

export function LandingPage() {
  return (
    <>
      <HeroSection />
      <LandingPrinciples />
      <LandingLivePreview />
      <LandingCta />
    </>
  );
}
