import { process_event, handle_response, view } from "shared";
import initCore from "shared";
import { writable } from "svelte/store";
import {
  EffectVariantRender,
  EffectVariantHttp,
  EffectVariantServerSentEvents,
  ViewModel,
  Request,
} from "shared_types/types/shared_types";
import type {
  Effect,
  Event,
  HttpResponse,
  SseResponse,
} from "shared_types/types/shared_types";
import {
  BincodeSerializer,
  BincodeDeserializer,
} from "shared_types/bincode/mod";
import { request as http } from "./http";
import { request as sse } from "./sse";
import { isTauri } from "@tauri-apps/api/core";
import {
  processEvent,
  handleResponse as tauriHandleResponse,
  view as tauriView,
} from "../../../../dist-js";

type Response = HttpResponse | SseResponse;

const { subscribe, set } = writable(new ViewModel("", false));

export async function update(event: Event) {
  if (isTauri()) {
    await tauriUpdate(event);
  } else {
    await wasmUpdate(event);
  }
}

export async function respond(id: number, response: Response) {
  const serializer = new BincodeSerializer();
  response.serialize(serializer);

  const effects = isTauri()
    ? await tauriHandleResponse(id, serializer.getBytes())
    : await handle_response(id, serializer.getBytes());

  if (effects) {
    const requests = deserializeRequests(effects);
    for (const { id, effect } of requests) {
      processEffect(id, effect);
    }
  }
}

const getView = async (): Promise<Uint8Array | null> => {
  if (isTauri()) {
    return await tauriView();
  } else {
    return view();
  }
};

const tauriUpdate = async (event: Event) => {
  const serializer = new BincodeSerializer();
  event.serialize(serializer);
  const effects = await processEvent(serializer.getBytes());
  if (effects) {
    console.log("got effects", effects);
    const requests = deserializeRequests(effects);
    console.log("got requests", requests);
    for (const { id, effect } of requests) {
      processEffect(id, effect);
    }
  } else {
    console.log("no effects ", effects);
  }
};

const wasmUpdate = async (event: Event) => {
  console.log("event", event);
  await initCore();

  const serializer = new BincodeSerializer();
  event.serialize(serializer);

  const effects = process_event(serializer.getBytes());
  const requests = deserializeRequests(effects);
  for (const { id, effect } of requests) {
    processEffect(id, effect);
  }
};

async function processEffect(id: number, effect: Effect) {
  console.log("effect", effect);
  switch (effect.constructor) {
    case EffectVariantRender: {
      getView().then((view) => view && updateView(deserializeView(view)));
      break;
    }
    case EffectVariantHttp: {
      const request = (effect as EffectVariantHttp).value;
      const response = await http(request);
      respond(id, response);
      break;
    }
    case EffectVariantServerSentEvents: {
      const request = (effect as EffectVariantServerSentEvents).value;
      for await (const response of sse(request)) {
        respond(id, response);
      }
      break;
    }
  }
}

export const updateView = (view: ViewModel) => {
  set(view);
};

function deserializeRequests(bytes: Uint8Array): Request[] {
  const deserializer = new BincodeDeserializer(bytes);
  const len = deserializer.deserializeLen();
  const requests: Request[] = [];
  for (let i = 0; i < len; i++) {
    const request = Request.deserialize(deserializer);
    requests.push(request);
  }
  return requests;
}

function deserializeView(bytes: Uint8Array): ViewModel {
  return ViewModel.deserialize(new BincodeDeserializer(bytes));
}

export default {
  subscribe,
};
