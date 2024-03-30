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
            selectedTab: Tab.CPU,
            setSelectedTab: (tab) => set({ selectedTab: tab }),
        }),
        {
            name: "tab-store",
        },
    ),
);
