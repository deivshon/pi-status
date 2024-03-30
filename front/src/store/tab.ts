import { Tab } from "@/models/app";
import { create } from "zustand";
import { persist } from "zustand/middleware";

interface TabStore {
    selectedTab: Tab;
    setSelectedTab: (tab: Tab) => void;
}

export const useTabStore = create<TabStore>()(
    persist(
        (set, _) => ({
            selectedTab: "cpu-tab",
            setSelectedTab: (tab) => set({ selectedTab: tab }),
        }),
        {
            name: "tab-store",
        },
    ),
);
