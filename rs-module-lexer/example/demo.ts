import { parse } from 'rs-module-lexer'

const result = parse({
  input: [
    {
      filename: 'index.ts',
      code: `
        export const member = 5
        import { useState } from 'react'
      `,
    },
    // ... other files
  ],
})

console.log(result)