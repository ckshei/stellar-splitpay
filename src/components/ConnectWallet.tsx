import { walletStore } from '../useWalletStore'
import { useStore } from 'zustand'

export function ConnectWallet() {
  const { address, isConnected, connect, disconnect } = useStore(walletStore)

  const handleClick = () => {
    if (isConnected) {
      disconnect()
    } else {
      // Replace with your actual wallet connection logic
      connect('example-address')
    }
  }

  return (
    <div>
      <button onClick={handleClick}>
        {isConnected ? 'Disconnect' : 'Connect Wallet'}
      </button>
      <div>
        {isConnected ? `Connected: ${address}` : 'Not connected'}
      </div>
    </div>
  )
} 