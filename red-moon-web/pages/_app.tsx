import ReactFlow from "react-flow-renderer";
import "normalize.css/normalize.css";
import "@blueprintjs/icons/lib/css/blueprint-icons.css";
import "@blueprintjs/core/lib/css/blueprint.css";
import type { AppProps } from "next/app";
import { Button, Navbar } from "@blueprintjs/core";

function MyApp({ Component, pageProps }: AppProps) {
    return (
        <div style={{ padding: 50 }}>
            <Navbar fixedToTop>
                <Navbar.Group>
                    <Navbar.Heading>Red Moon</Navbar.Heading>
                    <Navbar.Divider />
                    <a href={"testing"}>
                        <Button minimal text={"Testing"} />
                    </a>
                </Navbar.Group>
            </Navbar>
            <Component {...pageProps} />
        </div>
    );
}

export default MyApp;
