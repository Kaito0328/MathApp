export { getWasm } from './core/loader'
export * as bridge from './bridge'
export {
	solveLinearSystem2D,
	logisticFit2D,
	logisticPredictProbaFromModel,
	unpackGmmParamsHigh as unpackGmmParams,
	gmmFit2D,
	gmmPredictProbaFromPacked,
	bayesianLinearPosterior2D,
	kalmanPredict,
	kalmanUpdate,
	createNormal,
	toMatrix,
} from './api'
