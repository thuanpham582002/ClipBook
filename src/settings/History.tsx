import * as React from "react";
import {Label} from "@/components/ui/label";
import {Switch} from "@/components/ui/switch";
import {useEffect, useState} from "react";
import {
  CopyAndMergeSeparator,
  DoubleClickStrategy,
  NumberActionStrategy,
  prefGetClearHistoryOnMacReboot,
  prefGetClearHistoryOnQuit,
  prefGetCopyAndMergeEnabled,
  prefGetCopyAndMergeSeparator,
  prefGetCopyToClipboardAfterMerge,
  prefGetKeepFavoritesOnClearHistory,
  prefGetWarnOnClearHistory, prefIsClearHistoryOnMacRebootManaged, prefIsClearHistoryOnQuitManaged,
  prefIsKeepFavoritesOnClearHistoryManaged, prefIsShowPreviewForLinksManaged,
  prefIsWarnOnClearHistoryManaged,
  prefSetClearHistoryOnMacReboot,
  prefSetClearHistoryOnQuit,
  prefSetCopyAndMergeEnabled,
  prefSetCopyAndMergeSeparator,
  prefSetCopyOnDoubleClick,
  prefSetCopyOnNumberAction,
  prefSetCopyToClipboardAfterMerge,
  prefSetKeepFavoritesOnClearHistory,
  prefSetPasteOnClick,
  prefSetShowPreviewForLinks,
  prefSetTreatDigitNumbersAsColor,
  prefSetUpdateHistoryAfterAction,
  prefSetWarnOnClearHistory,
  prefShouldCopyOnDoubleClick,
  prefShouldCopyOnNumberAction,
  prefShouldPasteOnClick,
  prefShouldShowPreviewForLinks,
  prefShouldTreatDigitNumbersAsColor,
  prefShouldUpdateHistoryAfterAction,
} from "@/pref";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select"
import {
  DropdownMenu,
  DropdownMenuContent, DropdownMenuRadioGroup,
  DropdownMenuRadioItem,
  DropdownMenuTrigger
} from "@/components/ui/dropdown-menu";
import {ChevronsUpDown, Download, Upload} from "lucide-react";
import {Button} from "@/components/ui/button";
import { Trans, useTranslation } from 'react-i18next';
import {exportAllData, importData, BackupData} from "@/db";

declare const closeSettingsWindow: () => void;

export default function History() {
  const { t } = useTranslation();

  const doubleClickStrategyLabels = {
    [DoubleClickStrategy.COPY]: t('settings.history.doubleClick.copy'),
    [DoubleClickStrategy.PASTE]: t('settings.history.doubleClick.paste'),
  }

  const numberActionStrategyLabels = {
    [NumberActionStrategy.COPY]: t('settings.history.numberAction.copy'),
    [NumberActionStrategy.PASTE]: t('settings.history.numberAction.paste'),
  }

  const [warnOnClearHistory, setWarnOnClearHistory] = useState(prefGetWarnOnClearHistory())
  const [keepFavoritesOnClearHistory, setKeepFavoritesOnClearHistory] = useState(prefGetKeepFavoritesOnClearHistory())
  const [copyAndMergeEnabled, setCopyAndMergeEnabled] = useState(prefGetCopyAndMergeEnabled())
  const [copyToClipboardAfterMerge, setCopyToClipboardAfterMerge] = useState(prefGetCopyToClipboardAfterMerge())
  const [copyAndMergeSeparator, setCopyAndMergeSeparator] = useState(prefGetCopyAndMergeSeparator())
  const [clearHistoryOnQuit, setClearHistoryOnQuit] = useState(prefGetClearHistoryOnQuit())
  const [clearHistoryOnMacReboot, setClearHistoryOnMacReboot] = useState(prefGetClearHistoryOnMacReboot())
  const [treatDigitNumbersAsColor, setTreatDigitNumbersAsColor] = useState(prefShouldTreatDigitNumbersAsColor())
  const [showPreviewForLinks, setShowPreviewForLinks] = useState(prefShouldShowPreviewForLinks())
  const [updateHistoryAfterAction, setUpdateHistoryAfterAction] = useState(prefShouldUpdateHistoryAfterAction())
  const [pasteOnClick, setPasteOnClick] = useState(prefShouldPasteOnClick())
  const [doubleClickStrategy, setDoubleClickStrategy] = useState(prefShouldCopyOnDoubleClick() ? DoubleClickStrategy.COPY : DoubleClickStrategy.PASTE)
  const [numberActionStrategy, setNumberActionStrategy] = useState(prefShouldCopyOnNumberAction() ? NumberActionStrategy.COPY : NumberActionStrategy.PASTE)
  const [isExporting, setIsExporting] = useState(false)
  const [isImporting, setIsImporting] = useState(false)

  useEffect(() => {
    const down = (e: KeyboardEvent) => {
      if (e.key === "Escape") {
        closeSettingsWindow()
        e.preventDefault()
      }
    }

    document.addEventListener("keydown", down)
    return () => document.removeEventListener("keydown", down)
  }, [])

  function handleWarnOnClearHistoryChange(warnOnClearHistory: boolean) {
    setWarnOnClearHistory(warnOnClearHistory)
    prefSetWarnOnClearHistory(warnOnClearHistory)
  }

  function handleKeepFavoritesOnClearHistoryChange(keepFavoritesOnClearHistory: boolean) {
    setKeepFavoritesOnClearHistory(keepFavoritesOnClearHistory)
    prefSetKeepFavoritesOnClearHistory(keepFavoritesOnClearHistory)
  }

  function handleCopyAndMergeChange(copyAndMerge: boolean) {
    setCopyAndMergeEnabled(copyAndMerge)
    prefSetCopyAndMergeEnabled(copyAndMerge)
  }

  function handleCopyToClipboardAfterMergeChange(copyToClipboardAfterMerge: boolean) {
    setCopyToClipboardAfterMerge(copyToClipboardAfterMerge)
    prefSetCopyToClipboardAfterMerge(copyToClipboardAfterMerge)
  }

  function handleCopyAndMergeSeparatorChange(separator: CopyAndMergeSeparator) {
    setCopyAndMergeSeparator(separator)
    prefSetCopyAndMergeSeparator(separator)
  }

  function handleClearHistoryOnQuitChange(clearHistoryOnQuit: boolean) {
    setClearHistoryOnQuit(clearHistoryOnQuit)
    prefSetClearHistoryOnQuit(clearHistoryOnQuit)
  }

  function handleClearHistoryOnMacRebootChange(clearHistoryOnMacReboot: boolean) {
    setClearHistoryOnMacReboot(clearHistoryOnMacReboot)
    prefSetClearHistoryOnMacReboot(clearHistoryOnMacReboot)
  }

  function handleTreatDigitNumbersAsColorChange(treatDigitNumbersAsColor: boolean) {
    setTreatDigitNumbersAsColor(treatDigitNumbersAsColor)
    prefSetTreatDigitNumbersAsColor(treatDigitNumbersAsColor)
  }

  function handleShowPreviewForLinksChange(showPreviewForLinks: boolean) {
    setShowPreviewForLinks(showPreviewForLinks)
    prefSetShowPreviewForLinks(showPreviewForLinks)
  }

  function handleUpdateHistoryAfterActionChange(updateHistoryAfterAction: boolean) {
    setUpdateHistoryAfterAction(updateHistoryAfterAction)
    prefSetUpdateHistoryAfterAction(updateHistoryAfterAction)
  }

  function handlePasteOnClickChange(pasteOnClick: boolean) {
    setPasteOnClick(pasteOnClick)
    prefSetPasteOnClick(pasteOnClick)
  }

  function handleDoubleClickStrategyChange(doubleClickStrategy: string) {
    setDoubleClickStrategy(doubleClickStrategy as DoubleClickStrategy)
    prefSetCopyOnDoubleClick(doubleClickStrategy === DoubleClickStrategy.COPY)
  }

  function handleNumberActionStrategyChange(numberActionStrategy: string) {
    setNumberActionStrategy(numberActionStrategy as NumberActionStrategy)
    prefSetCopyOnNumberAction(numberActionStrategy === NumberActionStrategy.COPY)
  }

  async function handleExportData() {
    try {
      setIsExporting(true)
      const backupData = await exportAllData()
      
      // Create download
      const blob = new Blob([JSON.stringify(backupData, null, 2)], { type: 'application/json' })
      const url = URL.createObjectURL(blob)
      
      // Create filename with timestamp
      const timestamp = new Date().toISOString().replace(/[:.]/g, '-').slice(0, -5)
      const filename = `clipbook-backup-${timestamp}.json`
      
      // Download file
      const a = document.createElement('a')
      a.href = url
      a.download = filename
      document.body.appendChild(a)
      a.click()
      document.body.removeChild(a)
      URL.revokeObjectURL(url)
      
      console.log(`✅ Exported ${backupData.clips.length} clips to ${filename}`)
    } catch (error) {
      console.error('❌ Export failed:', error)
      alert(`Export failed: ${error}`)
    } finally {
      setIsExporting(false)
    }
  }

  async function handleImportData() {
    try {
      setIsImporting(true)
      
      // Create file input
      const input = document.createElement('input')
      input.type = 'file'
      input.accept = '.json'
      
      input.onchange = async (e) => {
        const file = (e.target as HTMLInputElement).files?.[0]
        if (!file) return
        
        try {
          const text = await file.text()
          const backupData: BackupData = JSON.parse(text)
          
          // Confirm before importing
          const shouldMerge = confirm(`Import ${backupData.clips.length} clips?\n\nOK = Merge with existing data\nCancel = Replace all data`)
          if (shouldMerge === null) return // User cancelled
          
          await importData(backupData, { merge: shouldMerge })
          
          alert(`✅ Successfully imported ${backupData.clips.length} clips!`)
          
          // Refresh page to show new data
          if (typeof window !== 'undefined') {
            window.location.reload()
          }
        } catch (error) {
          console.error('❌ Import failed:', error)
          alert(`Import failed: ${error}`)
        }
      }
      
      input.click()
    } catch (error) {
      console.error('❌ Import failed:', error)
      alert(`Import failed: ${error}`)
    } finally {
      setIsImporting(false)
    }
  }

  return (
      <div className="flex h-screen select-none">
        <div className="flex flex-col flex-grow">
          <div className="flex pt-8 px-8 border-b border-b-border draggable sticky">
            <span className="text-2xl pb-3 font-semibold">{t('settings.history.title')}</span>
          </div>

          <div className="flex flex-col px-8 pb-6 gap-4 flex-grow overflow-y-auto scrollbar-thin scrollbar-thumb-scrollbar scrollbar-track-transparent">
            <div className="flex items-center justify-between space-x-20 pt-6 pb-1">
              <Label htmlFor="updateHistoryAfterAction" className="flex flex-col text-base">
                <span className="">{t('settings.history.updateHistoryAfterAction.title')}</span>
                <span className="text-neutral-500 font-normal text-sm mt-1">
                  {t('settings.history.updateHistoryAfterAction.description')}
                </span>
              </Label>
              <Switch id="updateHistoryAfterAction" checked={updateHistoryAfterAction}
                      onCheckedChange={handleUpdateHistoryAfterActionChange}/>
            </div>

            <hr/>

            <div className="flex items-center justify-between space-x-20 pb-1">
              <Label htmlFor="pasteOnClick" className="flex flex-col text-base">
                <span className="">{t('settings.history.pasteOnClick.title')}</span>
                <span className="text-neutral-500 font-normal text-sm mt-1">
                  {t('settings.history.pasteOnClick.description')}
                </span>
              </Label>
              <Switch id="pasteOnClick" checked={pasteOnClick}
                      onCheckedChange={handlePasteOnClickChange}/>
            </div>

            <div className="flex items-center justify-between space-x-10 pb-1">
              <Label htmlFor="pasteOnClick" className="flex flex-col text-base">
                <span className="">{t('settings.history.doubleClick.title')}</span>
                <span className="text-neutral-500 font-normal text-sm mt-1">
                  {t('settings.history.doubleClick.description')}
                </span>
              </Label>
              <DropdownMenu>
                <DropdownMenuTrigger asChild>
                  <Button variant="outline" size="dropdown" className="px-4 outline-none">
                    {doubleClickStrategyLabels[doubleClickStrategy]}
                    <ChevronsUpDown className="ml-2 h-4 w-4 shrink-0 opacity-50"/>
                  </Button>
                </DropdownMenuTrigger>
                <DropdownMenuContent className="p-1.5 bg-actions-background" align="end">
                  <DropdownMenuRadioGroup value={doubleClickStrategy}
                                          onValueChange={handleDoubleClickStrategyChange}>
                    <DropdownMenuRadioItem value={DoubleClickStrategy.PASTE}
                                           className="py-2 pr-4 pl-10">
                      <div className="flex flex-col">
                        <span>{doubleClickStrategyLabels[DoubleClickStrategy.PASTE]}</span>
                      </div>
                    </DropdownMenuRadioItem>
                    <DropdownMenuRadioItem value={DoubleClickStrategy.COPY}
                                           className="py-2 pr-4 pl-10">
                      <div className="flex flex-col">
                        <span>{doubleClickStrategyLabels[DoubleClickStrategy.COPY]}</span>
                      </div>
                    </DropdownMenuRadioItem>

                  </DropdownMenuRadioGroup>
                </DropdownMenuContent>
              </DropdownMenu>
            </div>

            <div className="flex items-center justify-between space-x-12 pb-1">
              <Label htmlFor="pasteOnClick" className="flex flex-col text-base">
                <span className="">{t('settings.history.numberAction.title')}</span>
                <span className="text-neutral-500 font-normal text-sm mt-1">
                  {t('settings.history.numberAction.description')}
                </span>
              </Label>
              <DropdownMenu>
                <DropdownMenuTrigger asChild>
                  <Button variant="outline" size="dropdown" className="px-4 outline-none">
                    {numberActionStrategyLabels[numberActionStrategy]}
                    <ChevronsUpDown className="ml-2 h-4 w-4 shrink-0 opacity-50"/>
                  </Button>
                </DropdownMenuTrigger>
                <DropdownMenuContent className="p-1.5 bg-actions-background" align="end">
                  <DropdownMenuRadioGroup value={numberActionStrategy}
                                          onValueChange={handleNumberActionStrategyChange}>
                    <DropdownMenuRadioItem value={NumberActionStrategy.PASTE}
                                           className="py-2 pr-4 pl-10">
                      <div className="flex flex-col">
                        <span>{numberActionStrategyLabels[NumberActionStrategy.PASTE]}</span>
                      </div>
                    </DropdownMenuRadioItem>
                    <DropdownMenuRadioItem value={NumberActionStrategy.COPY}
                                           className="py-2 pr-4 pl-10">
                      <div className="flex flex-col">
                        <span>{numberActionStrategyLabels[NumberActionStrategy.COPY]}</span>
                      </div>
                    </DropdownMenuRadioItem>
                  </DropdownMenuRadioGroup>
                </DropdownMenuContent>
              </DropdownMenu>
            </div>

            <hr/>

            <div className="flex items-center justify-between space-x-20 pb-1">
              <Label htmlFor="copyAndMerge" className="flex flex-col text-base">
                <span className="">{t('settings.history.copyAndMerge.title')}</span>
                <span className="text-neutral-500 font-normal text-sm mt-1">
                  <Trans 
                    i18nKey="settings.history.copyAndMerge.description"
                    components={{
                      kbd: <kbd />
                    }}
                  />
                </span>
              </Label>
              <Switch id="copyAndMerge" checked={copyAndMergeEnabled}
                      onCheckedChange={handleCopyAndMergeChange}/>
            </div>

            <div className="flex items-center justify-between space-x-10 py-1">
              <Label className="flex flex-col text-base">
                <span className="">{t('settings.history.copyAndMergeSeparator.title')}</span>
                <span className="text-neutral-500 font-normal text-sm">
                  {t('settings.history.copyAndMergeSeparator.description')}
                </span>
              </Label>
              <Select defaultValue={copyAndMergeSeparator}
                      onValueChange={handleCopyAndMergeSeparatorChange}
                      disabled={!copyAndMergeEnabled}>
                <SelectTrigger className="w-[150px]">
                  <SelectValue/>
                </SelectTrigger>
                <SelectContent>
                  <SelectItem value={CopyAndMergeSeparator.LINE}>{t('settings.history.copyAndMergeSeparator.line')}</SelectItem>
                  <SelectItem value={CopyAndMergeSeparator.SPACE}>{t('settings.history.copyAndMergeSeparator.space')}</SelectItem>
                </SelectContent>
              </Select>
            </div>

            <div className="flex items-center justify-between space-x-20 py-1">
              <Label htmlFor="copyToClipboardAfterMerge" className="flex flex-col text-base">
                <span className="">{t('settings.history.copyToClipboardAfterMerge.title')}</span>
                <span className="text-neutral-500 font-normal text-sm">
                  {t('settings.history.copyToClipboardAfterMerge.description')}
                </span>
              </Label>
              <Switch id="copyToClipboardAfterMerge"
                      checked={copyToClipboardAfterMerge}
                      onCheckedChange={handleCopyToClipboardAfterMergeChange}
                      disabled={!copyAndMergeEnabled}/>
            </div>

            <hr/>

            <div className="flex items-center justify-between space-x-12 pb-1">
              <Label htmlFor="digitToColor" className="flex flex-col text-base">
                <span className="">
                  {t('settings.history.treatDigitNumbersAsColor.title')}
                </span>
                <span className="text-neutral-500 font-normal text-sm mt-1">
                  {t('settings.history.treatDigitNumbersAsColor.description')}
                </span>
              </Label>
              <Switch id="digitToColor" checked={treatDigitNumbersAsColor}
                      onCheckedChange={handleTreatDigitNumbersAsColorChange}/>
            </div>

            <hr/>

            <div className="flex items-center justify-between space-x-12 pb-1">
              <Label htmlFor="showPreviewForLinks" className="flex flex-col text-base">
                <span className="">
                  {t('settings.history.showPreviewForLinks.title')}
                </span>
                <span className="text-neutral-500 font-normal text-sm mt-1">
                  {t('settings.history.showPreviewForLinks.description')}
                </span>
              </Label>
              <Switch id="showPreviewForLinks" checked={showPreviewForLinks}
                      onCheckedChange={handleShowPreviewForLinksChange}
                      disabled={prefIsShowPreviewForLinksManaged()}/>
            </div>

            <hr/>

            <div className="flex items-center justify-between space-x-20 py-1">
              <Label htmlFor="warnOnClearAll" className="flex flex-col text-base">
                <span className="">{t('settings.history.warnOnClearAll.title')}</span>
                <span className="text-neutral-500 font-normal text-sm">
                  {t('settings.history.warnOnClearAll.description')}
                </span>
              </Label>
              <Switch id="warnOnClearAll" checked={warnOnClearHistory}
                      onCheckedChange={handleWarnOnClearHistoryChange}
                      disabled={prefIsWarnOnClearHistoryManaged()}/>
            </div>

            <div className="flex items-center justify-between space-x-20 py-1">
              <Label htmlFor="keepFavoritesOnClearAll" className="flex flex-col text-base">
                <span className="">{t('settings.history.keepFavoritesOnClearAll.title')}</span>
                <span className="text-neutral-500 font-normal text-sm">
                  {t('settings.history.keepFavoritesOnClearAll.description')}
                </span>
              </Label>
              <Switch id="keepFavoritesOnClearAll" checked={keepFavoritesOnClearHistory}
                      onCheckedChange={handleKeepFavoritesOnClearHistoryChange}
                      disabled={prefIsKeepFavoritesOnClearHistoryManaged()}/>
            </div>

            <hr/>

            <div className="flex items-center justify-between space-x-20 py-1">
              <Label className="flex flex-col text-base">
                <span className="">Backup & Restore</span>
                <span className="text-neutral-500 font-normal text-sm">
                  Export your clips to backup file or import from backup
                </span>
              </Label>
              <div className="flex gap-2">
                <Button 
                  variant="outline" 
                  size="sm"
                  onClick={handleExportData}
                  disabled={isExporting}
                  className="flex items-center gap-2"
                >
                  <Download className="h-4 w-4" />
                  {isExporting ? 'Exporting...' : 'Export'}
                </Button>
                <Button 
                  variant="outline" 
                  size="sm"
                  onClick={handleImportData}
                  disabled={isImporting}
                  className="flex items-center gap-2"
                >
                  <Upload className="h-4 w-4" />
                  {isImporting ? 'Importing...' : 'Import'}
                </Button>
              </div>
            </div>

            <hr/>

            <div className="flex items-center justify-between space-x-20 py-1">
              <Label htmlFor="clearHistoryOnQuit" className="flex flex-col text-base">
                <span className="">{t('settings.history.clearHistoryOnQuit.title')}</span>
                <span className="text-neutral-500 font-normal text-sm">
                  {t('settings.history.clearHistoryOnQuit.description')}
                </span>
              </Label>
              <Switch id="clearHistoryOnQuit" checked={clearHistoryOnQuit}
                      onCheckedChange={handleClearHistoryOnQuitChange}
                      disabled={prefIsClearHistoryOnQuitManaged()}/>
            </div>
            <div className="flex items-center justify-between space-x-20 py-1">
              <Label htmlFor="clearHistoryOnMacReboot" className="flex flex-col text-base">
                <span className="">{t('settings.history.clearHistoryOnMacReboot.title')}</span>
                <span className="text-neutral-500 font-normal text-sm">
                  {t('settings.history.clearHistoryOnMacReboot.description')}
                </span>
              </Label>
              <Switch id="clearHistoryOnMacReboot" checked={clearHistoryOnMacReboot}
                      onCheckedChange={handleClearHistoryOnMacRebootChange}
                      disabled={prefIsClearHistoryOnMacRebootManaged()}/>
            </div>
          </div>
        </div>
      </div>
  )
}
