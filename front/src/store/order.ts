import { ProcessOrder, ProcessProperty } from "@/models/proc";
import { create } from "zustand";
import { persist } from "zustand/middleware";

interface OrderStore {
    order: ProcessOrder;
    setOrder: (order: ProcessOrder) => void;
}

export const useOrderStore = create<OrderStore>()(
    persist(
        (set, _) => ({
            order: {
                ord: ProcessProperty.Memory,
                rev: false,
            },
            setOrder: (order) => set({ order: order }),
        }),
        {
            name: "order-store",
        },
    ),
);
