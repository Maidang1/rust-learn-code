import { parse } from 'rs-module-lexer'

parse({
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