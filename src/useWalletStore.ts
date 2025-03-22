import { create } from 'zustand'
import { persist } from 'zustand/middleware'

interface WalletState {
  address: string | null
  isConnected: boolean
  connect: (address: string) => void
  disconnect: () => void
}

export const walletStore = create<WalletState>()(
  persist(
    (set) => ({
      address: null,
      isConnected: false,
      connect: (address) => set({ address, isConnected: true }),
      disconnect: () => set({ address: null, isConnected: false }),
    }),
    {
      name: 'wallet-storage'
    }
  )
)