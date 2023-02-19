import type { AppProps } from "next/app";
import Head from "next/head";
import Link from "next/link";
import "../styles/globals.css";

export default function App({ Component, pageProps }: AppProps) {
  return (
    <>
      <Head>
        <title>Nvim Lens</title>
        <meta name="viewport" content="width=device-width, initial-scale=1" />
        <link rel="icon" href="/favicon.ico" />
      </Head>
      <div className="max-w-screen-xl mx-auto">
        <Link href="/" className="text-xl font-medium mt-4 block">
          Nvim Lens
        </Link>
        <Component {...pageProps} />
      </div>
    </>
  );
}
