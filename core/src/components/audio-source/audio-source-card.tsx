"use client";

import { Button } from "@/components/ui/button";
import { useAudioSource } from "@/hooks/use-audio-source";
import { cn } from "@/lib/utils";
import { AuthDialog } from "@/components/auth-dialog.tsx";
import { useState } from "react";
import { AudioSource } from "@/lib/audio-sources";
import { invoke } from "@tauri-apps/api/core";
import { ApplicationResp } from "@/types/application.ts";
import {useTranslation} from "react-i18next";

export function AudioSourceCard({ source }: { source: AudioSource }) {
  const { audioSource, configureSource } = useAudioSource();
  const isSelected = audioSource?.find((s) => s.id === source.id)?.connected;
  const disabled = audioSource?.find((s) => s.id === source.id)?.disabled;
  const [isAuthDialogOpen, setIsAuthDialogOpen] = useState(false);
  const { t } = useTranslation();

  async function logout(): Promise<ApplicationResp<any> | undefined> {
    try {
      const res = await invoke<ApplicationResp<any>>("logout", {
        source: source.id,
      });
      console.log(res);
      return res;
    } catch (e) {
      console.error(e);
      return undefined;
    }
  }

  const handleConnectClick = () => {
    // 点击按钮时，打开 AuthDialog
    if (isSelected) {
      logout().then((res) => {
        if (res != undefined && res.code == 0) {
          // update current audio source connect status
          const newSource = audioSource?.map((s) => {
            if (s.id === source.id) {
              return { ...s, connected: false };
            }
            return s;
          });
          configureSource(newSource);
        }
      });
    } else {
      setIsAuthDialogOpen(true);
    }
  };

  return (
    <div
      className={cn(
        "flex items-center justify-between rounded-lg border p-4 transition-colors",
        isSelected && "border-primary bg-primary/5"
      )}
    >
      <div className="flex items-center gap-4">
        <div>
          <h3 className="font-medium">{source.name}</h3>
          <p className="text-sm text-muted-foreground">
            {t("setting_music_card_desc", { name: source.name })}
          </p>
        </div>
      </div>

      <Button
        variant={isSelected ? "outline" : "default"}
        onClick={handleConnectClick}
        disabled={disabled}
        className={cn(
          "w-fit gap-2 bg-transparent border-2 text-green-500 hover:bg-green-100", // Green border and text
          isSelected && "border-green-500 text-green-500" // When selected, green border and text
        )}
      >
        {isSelected ? t("disconnect") : t("connect")}
      </Button>

      <AuthDialog
        isOpen={isAuthDialogOpen}
        setIsOpen={setIsAuthDialogOpen}
        source={source}
      />
    </div>
  );
}
