import { WebSocketProvider } from '@ethersproject/providers'
import { providers } from 'ethers'

const getWsProvider = (): WebSocketProvider =>
  new providers.WebSocketProvider('ws://localhost:8546')

async function main() {
  const provider = getWsProvider()

  let count = 0
  provider.on('pending', async (txHash: string) => {
    const tx = await provider.getTransaction(txHash)
    // eslint-disable-next-line @typescript-eslint/no-unnecessary-condition
    if (!tx) {
      // console.error(`Abort: "tx null" hash=${txHash}`)
      return
    }
    count++
    console.log(count, txHash)
    return
  })
}
  
main().catch((error) => {
  console.error('[EXIT WITH ERROR]', error)
  process.exit(1)
})
