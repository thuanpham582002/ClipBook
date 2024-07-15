import '../App.css';
import {Button} from "@/components/ui/button";
import React from "react";
import Actions from "@/components/Actions";
import {
  prefGetCloseAppShortcut,
  prefGetPasteSelectedItemToActiveAppShortcut,
  prefGetSelectNextItemShortcut,
  prefGetSelectPreviousItemShortcut
} from "@/pref";
import ShortcutLabel from "@/components/ShortcutLabel";

type StatusBarProps = {
  appName: string
  onHideActions: () => void
}

export default function StatusBar(props: StatusBarProps) {
  return (
      <div
          className="flex items-center justify-between p-2 border-t-solid border-t-border border-t">
        <div className="flex space-x-1 text-sm text-primary-foreground">
          <Button variant="info" className="p-1 h-8 rounded-sm">
            <ShortcutLabel
                shortcut={prefGetSelectNextItemShortcut() + " + " + prefGetSelectPreviousItemShortcut()}/>
            <p className="px-2">Navigate</p>
          </Button>

          <Button variant="ghost" className="p-1 h-8 rounded-sm">
            <ShortcutLabel shortcut={prefGetPasteSelectedItemToActiveAppShortcut()}/>
            <p className="px-2 text-">Paste to {props.appName}</p>
          </Button>

          <Button variant="ghost" className="p-1 h-8 rounded-sm"
                  title="Close window (Escape or ⌘W)">
            <ShortcutLabel shortcut={prefGetCloseAppShortcut()}/>
            <p className="px-2">Close</p>
          </Button>
        </div>

        <div className="flex space-x-2">
          <Actions onHideActions={props.onHideActions}/>
        </div>
      </div>
  )
}
