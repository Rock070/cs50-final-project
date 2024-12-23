FROM node:iron-alpine AS builder
WORKDIR /app
COPY package.json pnpm-lock.yaml ./
RUN corepack enable
RUN pnpm install --frozen-lockfile
COPY . .
RUN pnpm run build
RUN pnpm prune --prod

FROM node:iron-alpine AS final
WORKDIR /app
COPY --from=builder /app/build ./build
COPY --from=builder /app/node_modules ./node_modules
COPY --from=builder /app/static ./static
COPY --from=builder /app/package.json ./package.json
ENV NODE_ENV=production
EXPOSE 3000
CMD ["node", "build"]