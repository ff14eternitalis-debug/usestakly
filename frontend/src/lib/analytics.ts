declare global {
  interface Window {
    umami?: {
      track: (
        eventName: string,
        eventData?: Record<string, string>
      ) => void;
    };
  }
}

const scriptUrl = import.meta.env.VITE_UMAMI_SCRIPT_URL;
const websiteId = import.meta.env.VITE_UMAMI_WEBSITE_ID;
const enabled = Boolean(scriptUrl) && Boolean(websiteId);

export function analyticsEnabled() {
  return enabled;
}

export function installUmamiScript() {
  if (
    !enabled ||
    typeof document === "undefined" ||
    document.querySelector("script[data-website-id]")
  ) {
    return;
  }

  const script = document.createElement("script");
  script.defer = true;
  script.src = scriptUrl;
  script.setAttribute("data-website-id", websiteId);
  document.head.appendChild(script);
}

export function trackEvent(
  eventName: string,
  eventData?: Record<string, string>
) {
  if (!enabled || typeof window === "undefined") return;
  window.umami?.track(eventName, eventData);
}

