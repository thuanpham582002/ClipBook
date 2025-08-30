declare const isActivated: () => boolean;
declare const isTrial: () => boolean;
declare const getTrialDaysLeft: () => number;

export function isLicenseActivated(): boolean {
  // Fallback for web development mode when native functions aren't available
  if (typeof isActivated === 'undefined') {
    return true; // Default to activated for development
  }
  return isActivated()
}

export function isTrialLicense(): boolean {
  // Fallback for web development mode when native functions aren't available
  if (typeof isTrial === 'undefined') {
    return false; // Default to not trial for development
  }
  return isTrial()
}

export function isTrialLicenseExpired(): boolean {
  // Fallback for web development mode when native functions aren't available
  if (typeof isTrial === 'undefined') {
    return false; // Default to not expired for development
  }
  return isTrial() && getTrialLicenseDaysLeft() <= 0
}

export function getTrialLicenseDaysLeft(): number {
  // Fallback for web development mode when native functions aren't available
  if (typeof getTrialDaysLeft === 'undefined') {
    return 30; // Default to 30 days for development
  }
  return getTrialDaysLeft()
}
