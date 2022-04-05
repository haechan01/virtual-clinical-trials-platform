import type { ApiPromise } from '@polkadot/api'
import { ContractPromise } from '@polkadot/api-contract'
import { create } from '@phala/sdk'
import { Button } from 'baseui/button'
import { FormControl } from 'baseui/form-control'
import { Input } from 'baseui/input'
import { toaster } from 'baseui/toast'
import { useAtom } from 'jotai'
import { atomWithStorage } from 'jotai/utils'
import { focusAtom } from 'jotai/optics'
import { useRef, VFC } from "react"
import useIsClient from '../hooks/useIsClient.ts'
import { createApi } from '../lib/polkadotApi.ts'

const endpointAtom = atomWithStorage<string>(
    'atom:endpoint',
    'wss://poc5.phala.network/ws'
)
const pruntimeURLAtom = atomWithStorage<string>(
    'atom:pruntime_url',
    'http://127.0.0.1:8000'
)
const contractsAtom = atomWithStorage<
    Record<string, { contractId: string}>
>('atom:contracts', {})

const ContractLoader: VFC<{
    name: string
    onLoad: (res: { api: ApiPromise; contract: ContractPromise }) => void
}> = ({ name, onLoad }) => {
    const contractInfoAtom = useRef(
        focusAtom(contractsAtom, (optic) => optic.prop(name))
    )
    const [contractInfo, setContractInfo] = useAtom(contractInfoAtom.current)
    const [endpoint, setEndpoint] = useAtom(endpointAtom)
    const [pruntimeURL, setPruntimeURL] = useAtom(pruntimeURLAtom)
    const metadata = require('../metadata.json')
    const { contractId = ''} = contractInfo || {}
    const isClient = useIsClient()
    if (!isClient) return null

    const loadContract = async () => {
        try {
            const api = await createApi(endpoint)
            const contract = new ContractPromise(
                await create({ api, baseURL: pruntimeURL, contractId }),
                JSON.parse(metadata),
                contractId
            )
            onLoad({ api, contract })
            toaster.positive('Contract loaded successfully', {})
        } catch (err) {
            toaster.negative((err as Error).message, {})
        }
    }

    return (
        <>
            <FormControl label="WS Endpoint">
                <Input
                    placeholder="ws://localhost:9944"
                    overrides={{
                        Input: {
                            style: {
                                fontFamily: 'monospace',
                            },
                        },
                    }}
                    value={endpoint}
                    onChange={(e) => setEndpoint(e.currentTarget.value)}
                ></Input>
            </FormControl>
            <FormControl label="Pruntime URL">
                <Input
                    placeholder="http://localhost:8000"
                    overrides={{
                        Input: {
                            style: {
                                fontFamily: 'monospace',
                            },
                        },
                    }}
                    value={pruntimeURL}
                    onChange={(e) => setPruntimeURL(e.currentTarget.value)}
                ></Input>
            </FormControl>
            <FormControl label="Contract Id">
                <Input
                    overrides={{
                        Input: {
                            style: {
                                fontFamily: 'monospace',
                            },
                        },
                    }}
                    value={contractId}
                    onChange={(e) =>
                        setContractInfo((contractInfo) => ({
                            ...contractInfo,
                            contractId: e.currentTarget.value,
                        }))
                    }
                ></Input>
            </FormControl>

            <Button disabled={!contractId} onClick={loadContract}>
                Load Contract
            </Button> 
        </>
    )
}

export default ContractLoader