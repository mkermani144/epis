export const config = {
  episServerUrl:
    import.meta.env.VITE_EPIS_SERVER_URL || "http://localhost:9999",
} as const;
