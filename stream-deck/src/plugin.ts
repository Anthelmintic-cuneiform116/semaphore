import streamDeck from "@elgato/streamdeck";
import { SemaphoreLight } from "./actions/semaphore-light.js";

// Register all actions before connecting
streamDeck.actions.registerAction(new SemaphoreLight());

// Connect to Stream Deck
streamDeck.connect();
