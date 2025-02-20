/* ./setup/shiki.ts */
import { defineShikiSetup } from '@slidev/types'
import witLanguage from '../wit.tmLanguage.json'

export default defineShikiSetup(() => {
  return {
    langs: [
      'js',
      'typescript',
      'rust',
      'markdown',
      'typescript',
      witLanguage,
      // ...
    ],
  }
})
