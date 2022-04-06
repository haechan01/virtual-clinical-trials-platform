import type {InjectedAccountWithMeta} from '@polkadot/extension-inject/types'
import {Signer} from '@polkadot/types/types'

let enablePolkadotExtensionCache: Promise<void>
export const enablePolkadotExtension = async (): Promise<void> => {
  if (enablePolkadotExtensionCache) return enablePolkadotExtensionCache

  enablePolkadotExtensionCache = (async () => {
    const {web3Enable} = await import('@polkadot/extension-dapp')
    const extensions = await web3Enable('Phala SDK Example')
    try {
      if (extensions.length === 0) {
        throw new Error(
          'No extension installed, or the user did not accept the authorization'
        )
      }
    }
    catch (e) {
      console.log(e)
    }
  })()

  return enablePolkadotExtensionCache
}

export const getSigner = async (
  account: InjectedAccountWithMeta
): Promise<Signer> => {
  await enablePolkadotExtension()
  const {web3FromSource} = await import('@polkadot/extension-dapp')
  console.log("web3 from Source works")
  console.log(account.meta.source)
  const injector = await web3FromSource(account.meta.source)
  console.log("Account Injector works")
  const signer = injector.signer
  console.log("Signer works")

  return signer
}
