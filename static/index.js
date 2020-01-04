import init, {mount} from './pkg/remap.js'

(async () => {
  await init()
  mount("map-root")
})()
