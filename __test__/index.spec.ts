import test from 'ava'

import { Toaster, Toast, POWERSHELL_APP_ID } from '../index'

test('sync function from native code', (t) => {
  t.is(POWERSHELL_APP_ID, '{1AC14E77-02E7-4E5D-B744-2EB1AE5198B7}\\WindowsPowerShell\\v1.0\\powershell.exe')
  if (process.env.CI) return t.pass()
  const toaster = new Toaster()
  const res = toaster.show(
    new Toast()
      .text1('Hell')
      .text2('World')
      .text3('Fk Windows')
      .duration(false)
      .expiresIn(10000)
      .tag('tag')
      .group('group')
      .scenario(1),
  )
  t.true(res)
})
