import HistoryPane from "@/app/HistoryPane";
import {ThemeProvider} from "@/app/ThemeProvider"
import {ClipboardService} from "@/services/clipboard";

import {useEffect, useState} from "react";
import {
  prefGetCloseAppShortcut,
  prefGetCloseAppShortcut2,
  prefGetCloseAppShortcut3, prefGetLanguage,
  prefGetZoomUIInShortcut,
  prefGetZoomUIOutShortcut, prefGetZoomUIResetShortcut
} from "@/pref";
import {isShortcutMatch} from "@/lib/shortcuts";
import {TooltipProvider} from "@/components/ui/tooltip";
import {emitter} from "@/actions";

// Tauri imports will be available when running in Tauri environment

declare const hideAppWindow: () => void;
declare const zoomIn: () => void;
declare const zoomOut: () => void;
declare const resetZoom: () => void;

export default function App() {
  const [appName, setAppName] = useState("")
  const [appIcon, setAppIcon] = useState("")
  const [isInitialized, setIsInitialized] = useState(false)
  const [error, setError] = useState<string | null>(null)

  useEffect(() => {
    const down = (e: KeyboardEvent) => {
      // Prevent leaving the history items with the tab key.
      if (e.code === "Tab") {
        e.preventDefault()
      }
      // Close the app window with the close app shortcut.
      if (isShortcutMatch(prefGetCloseAppShortcut(), e)
          || isShortcutMatch(prefGetCloseAppShortcut2(), e)
          || isShortcutMatch(prefGetCloseAppShortcut3(), e)) {
        hideAppWindow()
        e.preventDefault()
      }
      // Zoom in the UI with the zoom in shortcut.
      if (isShortcutMatch(prefGetZoomUIInShortcut(), e)) {
        handleZoomIn()
        e.preventDefault()
      }
      // Zoom out the UI with the zoom out shortcut.
      if (isShortcutMatch(prefGetZoomUIOutShortcut(), e)) {
        handleZoomOut()
        e.preventDefault()
      }
      // Reset zoom.
      if (isShortcutMatch(prefGetZoomUIResetShortcut(), e)) {
        handleResetZoom()
        e.preventDefault()
      }
    }

    document.addEventListener("keydown", down)
    return () => document.removeEventListener("keydown", down)
  }, [])

  useEffect(() => {
    emitter.on("ZoomIn", handleZoomIn)
    emitter.on("ZoomOut", handleZoomOut)
    emitter.on("ResetZoom", handleResetZoom)
    return () => {
      emitter.off("ZoomIn", handleZoomIn)
      emitter.off("ZoomOut", handleZoomOut)
      emitter.off("ResetZoom", handleResetZoom)
    };
  }, []);

  // Initialize Tauri services
  useEffect(() => {
    const initializeServices = async () => {
      try {
        // Check if we're running in Tauri environment first
        if (!window.__TAURI__) {
          console.log("Tauri environment not available - running in limited mode");
          setIsInitialized(true);
          return;
        }

        const clipboardService = ClipboardService.getInstance();
        
        // Initialize services and check system health
        await clipboardService.initializeServices();
        
        // Set up real-time clipboard monitoring
        clipboardService.onClipboardChange((item) => {
          emitter.emit("ClipboardChanged", item);
        });
        
        console.log("Tauri environment detected, services initialized");
        setIsInitialized(true);
      } catch (err) {
        console.error("Failed to initialize Tauri services:", err);
        setError(err instanceof Error ? err.message : "Failed to initialize services");
        
        // Fallback to basic functionality
        console.log("Running in fallback mode - Tauri services unavailable");
        setIsInitialized(true);
      }
    };

    initializeServices();
  }, []);

  function setActiveAppInfo(appName: string, appIcon: string): void {
    setAppName(appName)
    setAppIcon(appIcon)
  }

  function onDidAppWindowHide() {
    emitter.emit("NotifyAppWindowDidHide")
  }

  function handleZoomIn() {
    if (typeof zoomIn === 'function') {
      zoomIn()
    } else if (window.__TAURI__) {
      // Tauri window zoom in - will be implemented in Rust backend
      console.log("Zoom in requested - Tauri mode")
      // Future: window.__TAURI__.invoke('window_zoom_in')
    }
    emitter.emit("FocusSearchInput")
  }

  function handleZoomOut() {
    if (typeof zoomOut === 'function') {
      zoomOut()
    } else if (window.__TAURI__) {
      // Tauri window zoom out - will be implemented in Rust backend
      console.log("Zoom out requested - Tauri mode")
      // Future: window.__TAURI__.invoke('window_zoom_out')
    }
    emitter.emit("FocusSearchInput")
  }

  function handleResetZoom() {
    if (typeof resetZoom === 'function') {
      resetZoom()
    } else if (window.__TAURI__) {
      // Tauri window zoom reset - will be implemented in Rust backend
      console.log("Zoom reset requested - Tauri mode")
      // Future: window.__TAURI__.invoke('window_zoom_reset')
    }
    emitter.emit("FocusSearchInput")
  }

  // Attach the function to the window object
  (window as any).setActiveAppInfo = setActiveAppInfo;
  (window as any).onDidAppWindowHide = onDidAppWindowHide;

  if (error) {
    return (
      <ThemeProvider defaultTheme="system">
        <TooltipProvider delayDuration={250}>
          <div className="flex items-center justify-center h-screen bg-background">
            <div className="text-center p-6">
              <h1 className="text-2xl font-bold text-destructive mb-4">Service Initialization Error</h1>
              <p className="text-muted-foreground mb-4">{error}</p>
              <p className="text-sm text-muted-foreground">
                The application will run in limited mode. Some features may not work as expected.
              </p>
            </div>
          </div>
        </TooltipProvider>
      </ThemeProvider>
    )
  }

  if (!isInitialized) {
    return (
      <ThemeProvider defaultTheme="system">
        <TooltipProvider delayDuration={250}>
          <div className="flex items-center justify-center h-screen bg-background">
            <div className="text-center">
              <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-primary mx-auto mb-4"></div>
              <p className="text-muted-foreground">Initializing ClipBook services...</p>
            </div>
          </div>
        </TooltipProvider>
      </ThemeProvider>
    )
  }

  return (
      <ThemeProvider defaultTheme="system">
        <TooltipProvider delayDuration={250}>
          <HistoryPane appName={appName} appIcon={appIcon}/>
        </TooltipProvider>
      </ThemeProvider>
  )
}
