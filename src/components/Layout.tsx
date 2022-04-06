import {FC} from 'react'
import {ToasterContainer} from 'baseui/toast'
import {Block} from 'baseui/block'
import AccountSelect from './AccountSelect.tsx'
import useIsClient from '../hooks/useIsClient.ts'

const Layout: FC<{title?: string}> = ({title}) => {
  const isClient = useIsClient()

  return (
    <Block width="100%" maxWidth="768px" margin="0 auto" padding="0 16px 24px">
      <Block
        as="header"
        height="120px"
        display="flex"
        alignItems="center"
        justifyContent="space-between"
      >
        {isClient && <AccountSelect></AccountSelect>}
      </Block>
      <ToasterContainer
        placement="topRight"
        autoHideDuration={3000}
        overrides={{ToastBody: {style: {wordBreak: 'break-all'}}}}/>
    </Block>
  )
}

export default Layout