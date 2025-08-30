import {LanguageCode} from "@/data";

declare const saveLanguage: (language: string) => void;
declare const getLanguage: () => string;

declare const saveTheme: (theme: string) => void;
declare const getTheme: () => string;

declare const saveLicenseKey: (licenseKey: string) => void;
declare const getLicenseKey: () => string;

declare const saveOpenAtLogin: (openAtLogin: boolean) => void;
declare const shouldOpenAtLogin: () => boolean;
declare const isOpenAtLoginManaged: () => boolean;

declare const saveCheckForUpdatesAutomatically: (value: boolean) => void;
declare const shouldCheckForUpdatesAutomatically: () => boolean;
declare const isCheckForUpdatesAutomaticallyManaged: () => boolean;

declare const allowCheckForUpdates: () => boolean;

declare const saveWarnOnClearHistory: (warn: boolean) => void;
declare const shouldWarnOnClearHistory: () => boolean;
declare const isWarnOnClearHistoryManaged: () => boolean;

declare const saveKeepFavoritesOnClearHistory: (keep: boolean) => void;
declare const shouldKeepFavoritesOnClearHistory: () => boolean;
declare const isKeepFavoritesOnClearHistoryManaged: () => boolean;

declare const saveIgnoreTransientContent: (ignore: boolean) => void;
declare const saveIgnoreConfidentialContent: (ignore: boolean) => void;
declare const isIgnoreTransientContentManaged: () => boolean;

declare const shouldIgnoreTransientContent: () => boolean;
declare const shouldIgnoreConfidentialContent: () => boolean;
declare const isIgnoreConfidentialContentManaged: () => boolean;

declare const saveShowIconInMenuBar: (showIcon: boolean) => void;
declare const shouldShowIconInMenuBar: () => boolean;
declare const isShowIconInMenuBarManaged: () => boolean;

declare const setAppsToIgnore: (apps: string) => void;
declare const getAppsToIgnore: () => string;
declare const isAppsToIgnoreManaged: () => boolean;

declare const saveCopyAndMergeEnabled: (enabled: boolean) => void;
declare const isCopyAndMergeEnabled: () => boolean;
declare const isCopyAndMergeEnabledManaged: () => boolean;

declare const saveCopyAndMergeSeparator: (separator: string) => void;
declare const getCopyAndMergeSeparator: () => string;

declare const saveCopyToClipboardAfterMerge: (copy: boolean) => void;
declare const shouldCopyToClipboardAfterMerge: () => boolean;

declare const saveClearHistoryOnQuit: (clear: boolean) => void;
declare const shouldClearHistoryOnQuit: () => boolean;
declare const isClearHistoryOnQuitManaged: () => boolean;

declare const saveClearHistoryOnMacReboot: (clear: boolean) => void;
declare const shouldClearHistoryOnMacReboot: () => boolean;
declare const isClearHistoryOnMacRebootManaged: () => boolean;

declare const saveOpenWindowStrategy: (strategy: string) => void;
declare const getOpenWindowStrategy: () => string;
declare const isOpenWindowStrategyManaged: () => boolean;

declare const setTreatDigitNumbersAsColor: (treat: boolean) => void;
declare const shouldTreatDigitNumbersAsColor: () => boolean;
declare const isTreatDigitNumbersAsColorManaged: () => boolean;

declare const setShowPreviewForLinks: (show: boolean) => void;
declare const shouldShowPreviewForLinks: () => boolean;
declare const isShowPreviewForLinksManaged: () => boolean;

declare const setUpdateHistoryAfterAction: (update: boolean) => void;
declare const shouldUpdateHistoryAfterAction: () => boolean;
declare const isUpdateHistoryAfterActionManaged: () => boolean;

declare const shouldPasteOnClick: () => boolean;
declare const setPasteOnClick: (paste: boolean) => void;
declare const isPasteOnClickManaged: () => boolean;

declare const shouldPlaySoundOnCopy: () => boolean;
declare const setPlaySoundOnCopy: (play: boolean) => void;
declare const isPlaySoundOnCopyManaged: () => boolean;

declare const shouldAlwaysDisplay: () => boolean;
declare const setAlwaysDisplay: (display: boolean) => void;

declare const setFeedbackProvided: (provided: boolean) => void;
declare const isFeedbackProvided: () => boolean;

declare const setCopyOnDoubleClick: (copy: boolean) => void;
declare const shouldCopyOnDoubleClick: () => boolean;
declare const isCopyOnDoubleClickManaged: () => boolean;

declare const setCopyOnNumberAction: (copy: boolean) => void;
declare const shouldCopyOnNumberAction: () => boolean;
declare const isCopyOnNumberActionManaged: () => boolean;

declare const saveOpenAppShortcut: (shortcut: string) => void;
declare const getOpenAppShortcut: () => string;
declare const saveCloseAppShortcut: (shortcut: string) => void;
declare const getCloseAppShortcut: () => string;
declare const saveCloseAppShortcut2: (shortcut: string) => void;
declare const getCloseAppShortcut2: () => string;
declare const saveCloseAppShortcut3: (shortcut: string) => void;
declare const getCloseAppShortcut3: () => string;
declare const saveSelectNextItemShortcut: (shortcut: string) => void;
declare const getSelectNextItemShortcut: () => string;
declare const saveSelectPreviousItemShortcut: (shortcut: string) => void;
declare const getSelectPreviousItemShortcut: () => string;
declare const savePasteSelectedItemToActiveAppShortcut: (shortcut: string) => void;
declare const getPasteSelectedItemToActiveAppShortcut: () => string;
declare const savePasteSelectedObjectToActiveAppShortcut: (shortcut: string) => void;
declare const getPasteSelectedObjectToActiveAppShortcut: () => string;
declare const savePasteNextItemShortcut: (shortcut: string) => void;
declare const getPasteNextItemShortcut: () => string;
declare const saveEditHistoryItemShortcut: (shortcut: string) => void;
declare const getEditHistoryItemShortcut: () => string;
declare const saveOpenInBrowserShortcut: (shortcut: string) => void;
declare const getOpenInBrowserShortcut: () => string;
declare const saveShowInFinderShortcut: (shortcut: string) => void;
declare const getShowInFinderShortcut: () => string;
declare const saveQuickLookShortcut: (shortcut: string) => void;
declare const getQuickLookShortcut: () => string;
declare const saveOpenInDefaultAppShortcut: (shortcut: string) => void;
declare const getOpenInDefaultAppShortcut: () => string;
declare const saveCopyToClipboardShortcut: (shortcut: string) => void;
declare const getCopyToClipboardShortcut: () => string;
declare const saveCopyObjectToClipboardShortcut: (shortcut: string) => void;
declare const getCopyObjectToClipboardShortcut: () => string;
declare const saveCopyTextFromImageShortcut: (shortcut: string) => void;
declare const getCopyTextFromImageShortcut: () => string;
declare const saveDeleteHistoryItemShortcut: (shortcut: string) => void;
declare const getDeleteHistoryItemShortcut: () => string;
declare const saveClearHistoryShortcut: (shortcut: string) => void;
declare const getClearHistoryShortcut: () => string;
declare const saveTogglePreviewShortcut: (shortcut: string) => void;
declare const getTogglePreviewShortcut: () => string;
declare const saveShowMoreActionsShortcut: (shortcut: string) => void;
declare const getShowMoreActionsShortcut: () => string;
declare const saveZoomUIInShortcut: (shortcut: string) => void;
declare const getZoomUIInShortcut: () => string;
declare const saveZoomUIOutShortcut: (shortcut: string) => void;
declare const getZoomUIOutShortcut: () => string;
declare const saveZoomUIResetShortcut: (shortcut: string) => void;
declare const getZoomUIResetShortcut: () => string;
declare const saveOpenSettingsShortcut: (shortcut: string) => void;
declare const getOpenSettingsShortcut: () => string;
declare const saveToggleFavoriteShortcut: (shortcut: string) => void;
declare const getToggleFavoriteShortcut: () => string;
declare const saveNavigateToFirstItemShortcut: (shortcut: string) => void;
declare const getNavigateToFirstItemShortcut: () => string;
declare const saveNavigateToLastItemShortcut: (shortcut: string) => void;
declare const getNavigateToLastItemShortcut: () => string;
declare const saveNavigateToNextGroupOfItemsShortcut: (shortcut: string) => void;
declare const getNavigateToNextGroupOfItemsShortcut: () => string;
declare const saveNavigateToPrevGroupOfItemsShortcut: (shortcut: string) => void;
declare const getNavigateToPrevGroupOfItemsShortcut: () => string;
declare const getSaveImageAsFileShortcut: () => string;
declare const saveSaveImageAsFileShortcut: (shortcut: string) => void;
declare const savePauseResumeShortcut: (shortcut: string) => void;
declare const getPauseResumeShortcut: () => string;
declare const saveRenameItemShortcut: (shortcut: string) => void;
declare const getRenameItemShortcut: () => string;
declare const saveMakeLowerCaseShortcut: (shortcut: string) => void;
declare const getMakeLowerCaseShortcut: () => string;
declare const saveMakeUpperCaseShortcut: (shortcut: string) => void;
declare const getMakeUpperCaseShortcut: () => string;
declare const saveCapitalizeShortcut: (shortcut: string) => void;
declare const getCapitalizeShortcut: () => string;
declare const saveSentenceCaseShortcut: (shortcut: string) => void;
declare const getSentenceCaseShortcut: () => string;
declare const saveRemoveEmptyLinesShortcut: (shortcut: string) => void;
declare const getRemoveEmptyLinesShortcut: () => string;
declare const saveStripAllWhitespacesShortcut: (shortcut: string) => void;
declare const getStripAllWhitespacesShortcut: () => string;
declare const saveTrimSurroundingWhitespacesShortcut: (shortcut: string) => void;
declare const getTrimSurroundingWhitespacesShortcut: () => string;
declare const saveToggleFilterShortcut: (shortcut: string) => void;
declare const getToggleFilterShortcut: () => string;

declare const isDeviceManaged: () => boolean;

export enum OpenWindowStrategy {
  ACTIVE_SCREEN_LAST_POSITION = "activeScreenLastPosition",
  ACTIVE_SCREEN_CENTER = "activeScreenCenter",
  ACTIVE_WINDOW_CENTER = "activeWindowCenter",
  SCREEN_WITH_CURSOR = "screenWithCursor",
  MOUSE_CURSOR = "mouseCursor",
  INPUT_CURSOR = "inputCursor",
}

export enum DoubleClickStrategy {
  COPY = "copy",
  PASTE = "paste",
}

export enum NumberActionStrategy {
  COPY = "copy",
  PASTE = "paste",
}

export function prefIsDeviceManaged() {
  if (typeof isDeviceManaged === 'undefined') return false
  return isDeviceManaged()
}

export function prefSetLanguage(language: LanguageCode) {
  if (typeof saveLanguage === 'undefined') return
  saveLanguage(language)
}

export function prefGetLanguage() : LanguageCode {
  // Fallback for web development mode when native functions aren't available
  if (typeof getLanguage === 'undefined') {
    return LanguageCode.EN_US; // Default to English US
  }
  return getLanguage() as LanguageCode
}

export function prefGetTheme() {
  if (typeof getTheme === 'undefined') return 'auto'
  return getTheme()
}

export function prefSetTheme(theme: string) {
  if (typeof saveTheme === 'undefined') return
  saveTheme(theme)
}

export function prefSetLicenseKey(licenseKey: string) {
  if (typeof saveLicenseKey === 'undefined') return
  saveLicenseKey(licenseKey)
}

export function prefGetLicenseKey() {
  if (typeof getLicenseKey === 'undefined') return ""
  return getLicenseKey()
}

export function prefGetOpenAtLogin() {
  if (typeof shouldOpenAtLogin === 'undefined') return false
  return shouldOpenAtLogin()
}

export function prefSetOpenAtLogin(openAtLogin: boolean) {
  if (typeof saveOpenAtLogin === 'undefined') return
  saveOpenAtLogin(openAtLogin)
}

export function prefIsOpenAtLoginManaged() {
  if (typeof isOpenAtLoginManaged === 'undefined') return false
  return isOpenAtLoginManaged()
}

export function prefGetCheckForUpdatesAutomatically() {
  if (typeof shouldCheckForUpdatesAutomatically === 'undefined') return true
  return shouldCheckForUpdatesAutomatically()
}

export function prefSetCheckForUpdatesAutomatically(checkForUpdatesAutomatically: boolean) {
  if (typeof saveCheckForUpdatesAutomatically === 'undefined') return
  saveCheckForUpdatesAutomatically(checkForUpdatesAutomatically)
}

export function prefIsCheckForUpdatesAutomaticallyManaged() {
  if (typeof isCheckForUpdatesAutomaticallyManaged === 'undefined') return false
  return isCheckForUpdatesAutomaticallyManaged()
}

export function prefAllowCheckForUpdates() {
  if (typeof allowCheckForUpdates === 'undefined') return true
  return allowCheckForUpdates()
}

export function prefGetWarnOnClearHistory() {
  if (typeof shouldWarnOnClearHistory === 'undefined') return true
  return shouldWarnOnClearHistory()
}

export function prefSetWarnOnClearHistory(warn: boolean) {
  if (typeof saveWarnOnClearHistory === 'undefined') return
  saveWarnOnClearHistory(warn)
}

export function prefIsWarnOnClearHistoryManaged() {
  if (typeof isWarnOnClearHistoryManaged === 'undefined') return false
  return isWarnOnClearHistoryManaged()
}

export function prefGetKeepFavoritesOnClearHistory() {
  if (typeof shouldKeepFavoritesOnClearHistory === 'undefined') return false
  return shouldKeepFavoritesOnClearHistory()
}

export function prefSetKeepFavoritesOnClearHistory(keep: boolean) {
  if (typeof saveKeepFavoritesOnClearHistory === 'undefined') return
  saveKeepFavoritesOnClearHistory(keep)
}

export function prefIsKeepFavoritesOnClearHistoryManaged() {
  if (typeof isKeepFavoritesOnClearHistoryManaged === 'undefined') return false
  return isKeepFavoritesOnClearHistoryManaged()
}

export function prefGetIgnoreTransientContent() {
  if (typeof shouldIgnoreTransientContent === 'undefined') return false
  return shouldIgnoreTransientContent()
}

export function prefSetIgnoreTransientContent(ignore: boolean) {
  if (typeof saveIgnoreTransientContent === 'undefined') return
  saveIgnoreTransientContent(ignore)
}

export function prefIsIgnoreTransientContentManaged() {
  if (typeof isIgnoreTransientContentManaged === 'undefined') return false
  return isIgnoreTransientContentManaged()
}

export function prefGetIgnoreConfidentialContent() {
  if (typeof shouldIgnoreConfidentialContent === 'undefined') return false
  return shouldIgnoreConfidentialContent()
}

export function prefSetIgnoreConfidentialContent(ignore: boolean) {
  if (typeof saveIgnoreConfidentialContent === 'undefined') return
  saveIgnoreConfidentialContent(ignore)
}

export function prefIsIgnoreConfidentialContentManaged() {
  if (typeof isIgnoreConfidentialContentManaged === 'undefined') return false
  return isIgnoreConfidentialContentManaged()
}

export function prefGetOpenAppShortcut() {
  if (typeof getOpenAppShortcut === 'undefined') return "Cmd+Shift+V"
  return getOpenAppShortcut()
}

export function prefSetOpenAppShortcut(shortcut: string) {
  if (typeof saveOpenAppShortcut === 'undefined') return
  saveOpenAppShortcut(shortcut)
}

export function prefGetCloseAppShortcut() {
  if (typeof getCloseAppShortcut === 'undefined') return "Escape"
  return getCloseAppShortcut()
}

export function prefSetCloseAppShortcut(shortcut: string) {
  if (typeof saveCloseAppShortcut === 'undefined') return
  saveCloseAppShortcut(shortcut)
}

export function prefGetCloseAppShortcut2() {
  if (typeof getCloseAppShortcut2 === 'undefined') return "Cmd+W"
  return getCloseAppShortcut2()
}

export function prefSetCloseAppShortcut2(shortcut: string) {
  if (typeof saveCloseAppShortcut2 === 'undefined') return
  saveCloseAppShortcut2(shortcut)
}

export function prefGetCloseAppShortcut3() {
  if (typeof getCloseAppShortcut3 === 'undefined') return "Cmd+Q"
  return getCloseAppShortcut3()
}

export function prefSetCloseAppShortcut3(shortcut: string) {
  if (typeof saveCloseAppShortcut3 === 'undefined') return
  saveCloseAppShortcut3(shortcut)
}

export function prefGetSelectNextItemShortcut() {
  // Fallback for web development mode when native functions aren't available
  if (typeof getSelectNextItemShortcut === 'undefined') {
    return 'ArrowDown'; // Default shortcut
  }
  return getSelectNextItemShortcut()
}

export function prefSetSelectNextItemShortcut(shortcut: string) {
  saveSelectNextItemShortcut(shortcut)
}

export function prefGetSelectPreviousItemShortcut() {
  // Fallback for web development mode when native functions aren't available
  if (typeof getSelectPreviousItemShortcut === 'undefined') {
    return 'ArrowUp'; // Default shortcut
  }
  return getSelectPreviousItemShortcut()
}

export function prefSetSelectPreviousItemShortcut(shortcut: string) {
  saveSelectPreviousItemShortcut(shortcut)
}

export function prefGetPasteSelectedItemToActiveAppShortcut() {
  // Fallback for web development mode when native functions aren't available
  if (typeof getPasteSelectedItemToActiveAppShortcut === 'undefined') {
    return ''; // Default to empty string
  }
  return getPasteSelectedItemToActiveAppShortcut()
}

export function prefSetPasteSelectedItemToActiveAppShortcut(shortcut: string) {
  savePasteSelectedItemToActiveAppShortcut(shortcut)
}

export function prefGetPasteSelectedObjectToActiveAppShortcut() {
  if (typeof getPasteSelectedObjectToActiveAppShortcut === 'undefined') return "Cmd+Shift+Enter"
  return getPasteSelectedObjectToActiveAppShortcut()
}

export function prefSetPasteSelectedObjectToActiveAppShortcut(shortcut: string) {
  if (typeof savePasteSelectedObjectToActiveAppShortcut === 'undefined') return
  savePasteSelectedObjectToActiveAppShortcut(shortcut)
}

export function prefGetPasteNextItemShortcut() {
  if (typeof getPasteNextItemShortcut === 'undefined') return "Cmd+N"
  return getPasteNextItemShortcut()
}

export function prefSetPasteNextItemShortcut(shortcut: string) {
  if (typeof savePasteNextItemShortcut === 'undefined') return
  savePasteNextItemShortcut(shortcut)
}

export function prefGetEditHistoryItemShortcut() {
  if (typeof getEditHistoryItemShortcut === 'undefined') return "Enter"
  return getEditHistoryItemShortcut()
}

export function prefSetEditHistoryItemShortcut(shortcut: string) {
  if (typeof saveEditHistoryItemShortcut === 'undefined') return
  saveEditHistoryItemShortcut(shortcut)
}

export function prefGetOpenInBrowserShortcut() {
  if (typeof getOpenInBrowserShortcut === 'undefined') return "Cmd+O"
  return getOpenInBrowserShortcut()
}

export function prefSetOpenInBrowserShortcut(shortcut: string) {
  if (typeof saveOpenInBrowserShortcut === 'undefined') return
  saveOpenInBrowserShortcut(shortcut)
}

export function prefGetShowInFinderShortcut() {
  if (typeof getShowInFinderShortcut === 'undefined') return "Cmd+Shift+F"
  return getShowInFinderShortcut()
}

export function prefSetShowInFinderShortcut(shortcut: string) {
  if (typeof saveShowInFinderShortcut === 'undefined') return
  saveShowInFinderShortcut(shortcut)
}

export function prefGetQuickLookShortcut() {
  if (typeof getQuickLookShortcut === 'undefined') return "Space"
  return getQuickLookShortcut()
}

export function prefSetQuickLookShortcut(shortcut: string) {
  if (typeof saveQuickLookShortcut === 'undefined') return
  saveQuickLookShortcut(shortcut)
}

export function prefGetOpenInDefaultAppShortcut() {
  if (typeof getOpenInDefaultAppShortcut === 'undefined') return "Cmd+Enter"
  return getOpenInDefaultAppShortcut()
}

export function prefSetOpenInDefaultAppShortcut(shortcut: string) {
  saveOpenInDefaultAppShortcut(shortcut)
}

export function prefGetCopyToClipboardShortcut() {
  if (typeof getCopyToClipboardShortcut === 'undefined') return 'Cmd+C'
  return getCopyToClipboardShortcut()
}

export function prefSetCopyToClipboardShortcut(shortcut: string) {
  if (typeof saveCopyToClipboardShortcut === 'undefined') return
  saveCopyToClipboardShortcut(shortcut)
}

export function prefGetCopyObjectToClipboardShortcut() {
  if (typeof getCopyObjectToClipboardShortcut === 'undefined') return 'Cmd+Shift+C'
  return getCopyObjectToClipboardShortcut()
}

export function prefSetCopyObjectToClipboardShortcut(shortcut: string) {
  if (typeof saveCopyObjectToClipboardShortcut === 'undefined') return
  saveCopyObjectToClipboardShortcut(shortcut)
}

export function prefGetDeleteHistoryItemShortcut() {
  if (typeof getDeleteHistoryItemShortcut === 'undefined') return 'Delete'
  return getDeleteHistoryItemShortcut()
}

export function prefSetDeleteHistoryItemShortcut(shortcut: string) {
  if (typeof saveDeleteHistoryItemShortcut === 'undefined') return
  saveDeleteHistoryItemShortcut(shortcut)
}

export function prefGetClearHistoryShortcut() {
  if (typeof getClearHistoryShortcut === 'undefined') return 'Cmd+Shift+Delete'
  return getClearHistoryShortcut()
}

export function prefSetClearHistoryShortcut(shortcut: string) {
  if (typeof saveClearHistoryShortcut === 'undefined') return
  saveClearHistoryShortcut(shortcut)
}

export function prefGetTogglePreviewShortcut() {
  // Fallback for web development mode when native functions aren't available
  if (typeof getTogglePreviewShortcut === 'undefined') {
    return 'Cmd+P'; // Default shortcut for toggle preview
  }
  return getTogglePreviewShortcut()
}

export function prefSetTogglePreviewShortcut(shortcut: string) {
  if (typeof saveTogglePreviewShortcut === 'undefined') return
  saveTogglePreviewShortcut(shortcut)
}

export function prefGetShowMoreActionsShortcut() {
  if (typeof getShowMoreActionsShortcut === 'undefined') return 'Cmd+M'
  return getShowMoreActionsShortcut()
}

export function prefSetShowMoreActionsShortcut(shortcut: string) {
  if (typeof saveShowMoreActionsShortcut === 'undefined') return
  saveShowMoreActionsShortcut(shortcut)
}

export function prefGetZoomUIInShortcut() {
  if (typeof getZoomUIInShortcut === 'undefined') return 'Cmd++'
  return getZoomUIInShortcut()
}

export function prefSetZoomUIInShortcut(shortcut: string) {
  if (typeof saveZoomUIInShortcut === 'undefined') return
  saveZoomUIInShortcut(shortcut)
}

export function prefGetZoomUIOutShortcut() {
  if (typeof getZoomUIOutShortcut === 'undefined') return 'Cmd+-'
  return getZoomUIOutShortcut()
}

export function prefSetZoomUIOutShortcut(shortcut: string) {
  if (typeof saveZoomUIOutShortcut === 'undefined') return
  saveZoomUIOutShortcut(shortcut)
}

export function prefGetZoomUIResetShortcut() {
  if (typeof getZoomUIResetShortcut === 'undefined') return 'Cmd+0'
  return getZoomUIResetShortcut()
}

export function prefSetZoomUIResetShortcut(shortcut: string) {
  if (typeof saveZoomUIResetShortcut === 'undefined') return
  saveZoomUIResetShortcut(shortcut)
}

export function prefGetOpenSettingsShortcut() {
  if (typeof getOpenSettingsShortcut === 'undefined') return 'Cmd+,'
  return getOpenSettingsShortcut()
}

export function prefSetOpenSettingsShortcut(shortcut: string) {
  if (typeof saveOpenSettingsShortcut === 'undefined') return
  saveOpenSettingsShortcut(shortcut)
}

export function prefGetToggleFavoriteShortcut() {
  if (typeof getToggleFavoriteShortcut === 'undefined') return 'Cmd+F'
  return getToggleFavoriteShortcut()
}

export function prefSetToggleFavoriteShortcut(shortcut: string) {
  if (typeof saveToggleFavoriteShortcut === 'undefined') return
  saveToggleFavoriteShortcut(shortcut)
}

export function prefSetShowIconInMenuBar(showIcon: boolean) {
  if (typeof saveShowIconInMenuBar === 'undefined') return
  saveShowIconInMenuBar(showIcon)
}

export function prefGetShowIconInMenuBar() {
  if (typeof shouldShowIconInMenuBar === 'undefined') return true
  return shouldShowIconInMenuBar()
}

export function prefIsShowIconInMenuBarManaged() {
  if (typeof isShowIconInMenuBarManaged === 'undefined') return false
  return isShowIconInMenuBarManaged()
}

export function prefGetAppsToIgnore(): string[] {
  if (typeof getAppsToIgnore === 'undefined') return []
  let apps = getAppsToIgnore();
  if (apps === "") {
    return []
  }
  return apps.split(",")
}

export function prefSetAppsToIgnore(apps: string[]) {
  if (typeof setAppsToIgnore === 'undefined') return
  setAppsToIgnore(apps.join(","))
}

export function prefIsAppsToIgnoreManaged() {
  if (typeof isAppsToIgnoreManaged === 'undefined') return false
  return isAppsToIgnoreManaged()
}

export function prefGetCopyTextFromImageShortcut() {
  if (typeof getCopyTextFromImageShortcut === 'undefined') return 'Cmd+Shift+T'
  return getCopyTextFromImageShortcut()
}

export function prefSetCopyTextFromImageShortcut(shortcut: string) {
  if (typeof saveCopyTextFromImageShortcut === 'undefined') return
  saveCopyTextFromImageShortcut(shortcut)
}

export function prefGetCopyAndMergeEnabled() {
  if (typeof isCopyAndMergeEnabled === 'undefined') return false
  return isCopyAndMergeEnabled()
}

export function prefSetCopyAndMergeEnabled(enabled: boolean) {
  if (typeof saveCopyAndMergeEnabled === 'undefined') return
  saveCopyAndMergeEnabled(enabled)
}

export enum CopyAndMergeSeparator {
  LINE = "\n",
  SPACE = " ",
}

export function prefGetCopyAndMergeSeparator(): CopyAndMergeSeparator {
  if (typeof getCopyAndMergeSeparator === 'undefined') return CopyAndMergeSeparator.LINE
  let separator = getCopyAndMergeSeparator();
  if (separator === " ") {
    return CopyAndMergeSeparator.SPACE
  }
  return CopyAndMergeSeparator.LINE
}

export function prefSetCopyAndMergeSeparator(separator: CopyAndMergeSeparator) {
  if (typeof saveCopyAndMergeSeparator === 'undefined') return
  saveCopyAndMergeSeparator(separator)
}

export function prefGetCopyToClipboardAfterMerge() {
  if (typeof shouldCopyToClipboardAfterMerge === 'undefined') return false
  return shouldCopyToClipboardAfterMerge()
}

export function prefSetCopyToClipboardAfterMerge(copy: boolean) {
  if (typeof saveCopyToClipboardAfterMerge === 'undefined') return
  saveCopyToClipboardAfterMerge(copy)
}

export function prefGetQuickPasteModifier() {
  return "MetaLeft"
}

export function prefGetQuickPasteShortcuts(): string[] {
  const shortcuts = [];
  let modifier = prefGetQuickPasteModifier();
  for (let i = 1; i <= 9; i++) {
    shortcuts.push(`${modifier} + Digit${i}`)
  }
  return shortcuts
}

export function prefGetClearHistoryOnQuit() {
  if (typeof shouldClearHistoryOnQuit === 'undefined') return false
  return shouldClearHistoryOnQuit()
}

export function prefSetClearHistoryOnQuit(clear: boolean) {
  if (typeof saveClearHistoryOnQuit === 'undefined') return
  saveClearHistoryOnQuit(clear)
}

export function prefIsClearHistoryOnQuitManaged() {
  if (typeof isClearHistoryOnQuitManaged === 'undefined') return false
  return isClearHistoryOnQuitManaged()
}

export function prefGetClearHistoryOnMacReboot() {
  // Fallback for web development mode when native functions aren't available
  if (typeof shouldClearHistoryOnMacReboot === 'undefined') {
    return false; // Default to false
  }
  return shouldClearHistoryOnMacReboot()
}

export function prefSetClearHistoryOnMacReboot(clear: boolean) {
  if (typeof saveClearHistoryOnMacReboot === 'undefined') return
  saveClearHistoryOnMacReboot(clear)
}

export function prefIsClearHistoryOnMacRebootManaged() {
  if (typeof isClearHistoryOnMacRebootManaged === 'undefined') return false
  return isClearHistoryOnMacRebootManaged()
}

export function prefGetNavigateToFirstItemShortcut() {
  if (typeof getNavigateToFirstItemShortcut === 'undefined') return "Home"
  return getNavigateToFirstItemShortcut()
}

export function prefSetNavigateToFirstItemShortcut(shortcut: string) {
  if (typeof saveNavigateToFirstItemShortcut === 'undefined') return
  saveNavigateToFirstItemShortcut(shortcut)
}

export function prefGetNavigateToLastItemShortcut() {
  if (typeof getNavigateToLastItemShortcut === 'undefined') return "End"
  return getNavigateToLastItemShortcut()
}

export function prefSetNavigateToLastItemShortcut(shortcut: string) {
  if (typeof saveNavigateToLastItemShortcut === 'undefined') return
  saveNavigateToLastItemShortcut(shortcut)
}

export function prefGetNavigateToNextGroupOfItemsShortcut() {
  if (typeof getNavigateToNextGroupOfItemsShortcut === 'undefined') return "PageDown"
  return getNavigateToNextGroupOfItemsShortcut()
}

export function prefSetNavigateToNextGroupOfItemsShortcut(shortcut: string) {
  if (typeof saveNavigateToNextGroupOfItemsShortcut === 'undefined') return
  saveNavigateToNextGroupOfItemsShortcut(shortcut)
}

export function prefGetNavigateToPrevGroupOfItemsShortcut() {
  if (typeof getNavigateToPrevGroupOfItemsShortcut === 'undefined') return "PageUp"
  return getNavigateToPrevGroupOfItemsShortcut()
}

export function prefSetNavigateToPrevGroupOfItemsShortcut(shortcut: string) {
  if (typeof saveNavigateToPrevGroupOfItemsShortcut === 'undefined') return
  saveNavigateToPrevGroupOfItemsShortcut(shortcut)
}

export function prefGetOpenWindowStrategy(): OpenWindowStrategy {
  let strategy = getOpenWindowStrategy()
  if (strategy === null) {
    return OpenWindowStrategy.ACTIVE_SCREEN_LAST_POSITION
  }
  if (strategy === OpenWindowStrategy.ACTIVE_SCREEN_LAST_POSITION) {
    return OpenWindowStrategy.ACTIVE_SCREEN_LAST_POSITION
  } else if (strategy === OpenWindowStrategy.ACTIVE_SCREEN_CENTER) {
    return OpenWindowStrategy.ACTIVE_SCREEN_CENTER
  } else if (strategy === OpenWindowStrategy.ACTIVE_WINDOW_CENTER) {
    return OpenWindowStrategy.ACTIVE_WINDOW_CENTER
  } else if (strategy === OpenWindowStrategy.SCREEN_WITH_CURSOR) {
    return OpenWindowStrategy.SCREEN_WITH_CURSOR
  } else if (strategy === OpenWindowStrategy.MOUSE_CURSOR) {
    return OpenWindowStrategy.MOUSE_CURSOR
  } else if (strategy === OpenWindowStrategy.INPUT_CURSOR) {
    return OpenWindowStrategy.INPUT_CURSOR
  }
  return OpenWindowStrategy.ACTIVE_SCREEN_LAST_POSITION
}

export function prefSetOpenWindowStrategy(strategy: OpenWindowStrategy) {
  saveOpenWindowStrategy(strategy)
}

export function prefSetTreatDigitNumbersAsColor(treat: boolean) {
  setTreatDigitNumbersAsColor(treat)
}

export function prefShouldTreatDigitNumbersAsColor() {
  // Fallback for web development mode when native functions aren't available
  if (typeof shouldTreatDigitNumbersAsColor === 'undefined') {
    return true; // Default to true
  }
  return shouldTreatDigitNumbersAsColor()
}

export function prefSetShowPreviewForLinks(show: boolean) {
  setShowPreviewForLinks(show)
}

export function prefShouldShowPreviewForLinks() {
  if (typeof shouldShowPreviewForLinks === 'undefined') return true
  return shouldShowPreviewForLinks()
}

export function prefIsShowPreviewForLinksManaged() {
  if (typeof isShowPreviewForLinksManaged === 'undefined') return false
  return isShowPreviewForLinksManaged()
}

export function prefGetSaveImageAsFileShortcut() {
  if (typeof getSaveImageAsFileShortcut === 'undefined') return "Cmd+S"
  return getSaveImageAsFileShortcut()
}

export function prefSetSaveImageAsFileShortcut(shortcut: string) {
  if (typeof saveSaveImageAsFileShortcut === 'undefined') return
  saveSaveImageAsFileShortcut(shortcut)
}

export function prefGetPauseResumeShortcut() {
  if (typeof getPauseResumeShortcut === 'undefined') return "Cmd+P"
  return getPauseResumeShortcut()
}

export function prefSetPauseResumeShortcut(shortcut: string) {
  if (typeof savePauseResumeShortcut === 'undefined') return
  savePauseResumeShortcut(shortcut)
}

export function prefSetUpdateHistoryAfterAction(update: boolean) {
  setUpdateHistoryAfterAction(update)
}

export function prefShouldUpdateHistoryAfterAction() {
  if (typeof shouldUpdateHistoryAfterAction === 'undefined') return true
  return shouldUpdateHistoryAfterAction()
}

export function prefSetRenameItemShortcut(shortcut: string) {
  if (typeof saveRenameItemShortcut === 'undefined') return
  saveRenameItemShortcut(shortcut)
}

export function prefGetRenameItemShortcut() {
  if (typeof getRenameItemShortcut === 'undefined') return "F2"
  return getRenameItemShortcut()
}

export function prefIsFeedbackProvided() {
  // Fallback for web development mode when native functions aren't available
  if (typeof isFeedbackProvided === 'undefined') {
    return false; // Default to false
  }
  return isFeedbackProvided()
}

export function prefSetFeedbackProvided(provided: boolean) {
  setFeedbackProvided(provided)
}

export function prefShouldPasteOnClick() {
  if (typeof shouldPasteOnClick === 'undefined') return false
  return shouldPasteOnClick()
}

export function prefSetPasteOnClick(paste: boolean) {
  setPasteOnClick(paste)
}

export function prefShouldPlaySoundOnCopy() {
  if (typeof shouldPlaySoundOnCopy === 'undefined') return false
  return shouldPlaySoundOnCopy()
}

export function prefSetPlaySoundOnCopy(play: boolean) {
  setPlaySoundOnCopy(play)
}

export function prefIsPlaySoundOnCopyManaged() {
  if (typeof isPlaySoundOnCopyManaged === 'undefined') return false
  return isPlaySoundOnCopyManaged()
}

export function prefShouldAlwaysDisplay() {
  // Fallback for web development mode when native functions aren't available
  if (typeof shouldAlwaysDisplay === 'undefined') {
    return false; // Default to false
  }
  return shouldAlwaysDisplay()
}

export function prefSetAlwaysDisplay(display: boolean) {
  setAlwaysDisplay(display)
}

export function prefGetMakeLowerCaseShortcut() {
  if (typeof getMakeLowerCaseShortcut === 'undefined') return 'Cmd+L'
  return getMakeLowerCaseShortcut()
}

export function prefSetMakeLowerCaseShortcut(shortcut: string) {
  if (typeof saveMakeLowerCaseShortcut === 'undefined') return
  saveMakeLowerCaseShortcut(shortcut)
}

export function prefGetMakeUpperCaseShortcut() {
  if (typeof getMakeUpperCaseShortcut === 'undefined') return 'Cmd+U'
  return getMakeUpperCaseShortcut()
}

export function prefSetMakeUpperCaseShortcut(shortcut: string) {
  if (typeof saveMakeUpperCaseShortcut === 'undefined') return
  saveMakeUpperCaseShortcut(shortcut)
}

export function prefGetCapitalizeShortcut() {
  if (typeof getCapitalizeShortcut === 'undefined') return 'Cmd+Shift+C'
  return getCapitalizeShortcut()
}

export function prefSetCapitalizeShortcut(shortcut: string) {
  if (typeof saveCapitalizeShortcut === 'undefined') return
  saveCapitalizeShortcut(shortcut)
}

export function prefGetSentenceCaseShortcut() {
  if (typeof getSentenceCaseShortcut === 'undefined') return 'Cmd+S'
  return getSentenceCaseShortcut()
}

export function prefSetSentenceCaseShortcut(shortcut: string) {
  if (typeof saveSentenceCaseShortcut === 'undefined') return
  saveSentenceCaseShortcut(shortcut)
}

export function prefGetRemoveEmptyLinesShortcut() {
  if (typeof getRemoveEmptyLinesShortcut === 'undefined') return 'Cmd+R'
  return getRemoveEmptyLinesShortcut()
}

export function prefSetRemoveEmptyLinesShortcut(shortcut: string) {
  if (typeof saveRemoveEmptyLinesShortcut === 'undefined') return
  saveRemoveEmptyLinesShortcut(shortcut)
}

export function prefGetStripAllWhitespacesShortcut() {
  if (typeof getStripAllWhitespacesShortcut === 'undefined') return 'Cmd+W'
  return getStripAllWhitespacesShortcut()
}

export function prefSetStripAllWhitespacesShortcut(shortcut: string) {
  if (typeof saveStripAllWhitespacesShortcut === 'undefined') return
  saveStripAllWhitespacesShortcut(shortcut)
}

export function prefGetTrimSurroundingWhitespacesShortcut() {
  if (typeof getTrimSurroundingWhitespacesShortcut === 'undefined') return 'Cmd+T'
  return getTrimSurroundingWhitespacesShortcut()
}

export function prefSetTrimSurroundingWhitespacesShortcut(shortcut: string) {
  if (typeof saveTrimSurroundingWhitespacesShortcut === 'undefined') return
  saveTrimSurroundingWhitespacesShortcut(shortcut)
}

export function prefGetToggleFilterShortcut() {
  // Fallback for web development mode when native functions aren't available
  if (typeof getToggleFilterShortcut === 'undefined') {
    return 'Cmd+F'; // Default shortcut for filter toggle
  }
  return getToggleFilterShortcut()
}

export function prefSetToggleFilterShortcut(shortcut: string) {
  if (typeof saveToggleFilterShortcut === 'undefined') return
  saveToggleFilterShortcut(shortcut)
}

export function prefSetCopyOnDoubleClick(copy: boolean) {
  setCopyOnDoubleClick(copy)
}

export function prefShouldCopyOnDoubleClick() {
  if (typeof shouldCopyOnDoubleClick === 'undefined') return true
  return shouldCopyOnDoubleClick()
}

export function prefSetCopyOnNumberAction(copy: boolean) {
  setCopyOnNumberAction(copy)
}

export function prefShouldCopyOnNumberAction() {
  if (typeof shouldCopyOnNumberAction === 'undefined') return true
  return shouldCopyOnNumberAction()
}
