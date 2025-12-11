export const config = {
  episServerUrl:
    process.env.NEXT_PUBLIC_EPIS_SERVER_URL || "http://localhost:9999",
} as const;
