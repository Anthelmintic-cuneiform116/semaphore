import { action, SingletonAction } from "@elgato/streamdeck";
import type { WillAppearEvent, WillDisappearEvent, KeyAction } from "@elgato/streamdeck";
import { queryState, type LightState } from "../ipc.js";

const POLL_INTERVAL_MS = 500;
const VALID_STATES: ReadonlySet<string> = new Set(["green", "yellow", "red"]);

@action({ UUID: "com.semaphore.streamdeck.light" })
export class SemaphoreLight extends SingletonAction {
  /** Active key-action contexts keyed by their context ID. */
  readonly #active = new Map<string, KeyAction>();
  #timer: ReturnType<typeof setInterval> | undefined;
  #lastState: LightState = "unknown";

  override async onWillAppear(ev: WillAppearEvent): Promise<void> {
    const ctx = ev.action as KeyAction;
    this.#active.set(ctx.id, ctx);

    // Show the last known state immediately while polling catches up
    await this.#applyImage(ctx, this.#lastState);

    if (this.#timer === undefined) {
      this.#startPolling();
    }
  }

  override onWillDisappear(ev: WillDisappearEvent): void {
    this.#active.delete(ev.action.id);
    if (this.#active.size === 0) {
      this.#stopPolling();
    }
  }

  #startPolling(): void {
    void this.#poll();
    this.#timer = setInterval(() => void this.#poll(), POLL_INTERVAL_MS);
  }

  #stopPolling(): void {
    if (this.#timer !== undefined) {
      clearInterval(this.#timer);
      this.#timer = undefined;
    }
    this.#lastState = "unknown";
  }

  async #poll(): Promise<void> {
    const state = await queryState();
    if (state === this.#lastState) return;
    this.#lastState = state;
    for (const ctx of this.#active.values()) {
      await this.#applyImage(ctx, state);
    }
  }

  async #applyImage(ctx: KeyAction, state: LightState): Promise<void> {
    const img = VALID_STATES.has(state) ? state : "unknown";
    await ctx.setImage(`imgs/${img}.png`).catch(() => {
      // Action context may have been removed between poll and apply
    });
  }
}
