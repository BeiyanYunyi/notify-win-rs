import test from 'ava'

import { Toaster, POWERSHELL_APP_ID } from '../index'

test('sync function from native code', (t) => {
  const toaster = new Toaster()
  const res = toaster.noti({ text1: 'Hell', text2: 'World', text3: 'Fk windows' })
  t.true(res)
  t.is(POWERSHELL_APP_ID, '{1AC14E77-02E7-4E5D-B744-2EB1AE5198B7}\\WindowsPowerShell\\v1.0\\powershell.exe')
})
