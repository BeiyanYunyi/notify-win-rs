import test from 'ava'

import { noti } from '../index'

test('sync function from native code', (t) => {
  const res = noti({ title: 'Hell', text1: 'World', text2: 'Fk windows' })
  t.true(res)
})
