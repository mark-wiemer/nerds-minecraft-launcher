// Local-only analytics for debugging
// Events are logged to console but never sent to any server

const isDev = import.meta.env.DEV

export const initAnalytics = () => {
  if (isDev) {
    console.log('[Analytics] Initialized (local-only mode)')
  }
}

export const debugAnalytics = () => {
  console.log('[Analytics] Debug mode enabled')
}

export const optOutAnalytics = () => {
  // No-op in local-only mode
  if (isDev) {
    console.log('[Analytics] Opt-out requested (no-op in local-only mode)')
  }
}

export const optInAnalytics = () => {
  // No-op in local-only mode
  if (isDev) {
    console.log('[Analytics] Opt-in requested (no-op in local-only mode)')
  }
}

export const trackEvent = (eventName, properties) => {
  // Log events locally for debugging, never send to server
  if (isDev) {
    console.log(`[Analytics] Event: ${eventName}`, properties)
  }
}
