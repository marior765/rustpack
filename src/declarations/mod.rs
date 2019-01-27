
extern crate futures;
extern crate serde;

// use futures::*;

// pub enum Entry {
//     EntryDynamic, 
//     EntryStatic
// }

// pub Stringuct EntryDynamic {
//     || -> (EntryStatic | Promise<EntryStatic>::new())
// }

// type EntryStaticFunc = Fn() -> EntryStatic;

// type EntryPropmisStatic = promise<EntryStatic>;

// pub enum EntryDynamic {
//     EntryStaticFunc,
//     EntryPropmisStatic
// }

// pub enum EntryStatic {
//     EntryObject,
//     EntryItem
// }

// via the `definition` "NonEmptyArrayOfUniqueStringValues".
//  */
// export type NonEmptyArrayOfUniqueStringValues = String[];
// via the `definition` "EntryItem".
//  */
// export type EntryItem = String | NonEmptyArrayOfUniqueStringValues;
// via the `definition` "Externals".
//  */
// export type Externals =
// 	| ((
// 			context: String,
// 			request: String,
// 			callback: (err: Error, result: String) => void
// 	  ) => void)
// 	| ExternalItem
// 	| (
// 			| ((
// 					context: String,
// 					request: String,
// 					callback: (err: Error, result: String) => void
// 			  ) => void)
// 			| ExternalItem)[];
// via the `definition` "ExternalItem".
//  */
// export type ExternalItem =
// 	| String
// 	| {
// 			
// 			[k: String]:
// 				| String
// 				| {
// 						[k: String]: any;
// 				  }
// 				| ArrayOfStringValues
// 				| bool;
// 	  }
// 	| RegExp;
// via the `definition` "ArrayOfStringValues".
//  */
// export type ArrayOfStringValues = String[];
// 
//  * This interface was referenced by `WebpackOptions`'s JSON-Schema
//  * via the `definition` "RuleSetConditionOrConditions".
//  */
// export type RuleSetConditionOrConditions = RuleSetCondition | RuleSetConditions;
// via the `definition` "RuleSetCondition".
//  */
// export type RuleSetCondition =
// 	| RegExp
// 	| String
// 	| ((value: String) => bool)
// 	| RuleSetConditions
// 	| {
// 			
// 			and: RuleSetConditions;
// 			
// 			exclude: RuleSetConditionOrConditions;
// 			
// 			include: RuleSetConditionOrConditions;
// 			
// 			not: RuleSetConditions;
// 			
// 			or: RuleSetConditions;
// 			
// 			test: RuleSetConditionOrConditions;
// 	  };
// via the `definition` "RuleSetConditions".
//  */
// export type RuleSetConditions = RuleSetConditionsRecursive;
// 
//  * This interface was referenced by `WebpackOptions`'s JSON-Schema
//  * via the `definition` "RuleSetConditionOrConditionsAbsolute".
//  */
// export type RuleSetConditionOrConditionsAbsolute =
// 	| RuleSetConditionAbsolute
// 	| RuleSetConditionsAbsolute;
// via the `definition` "RuleSetConditionAbsolute".
//  */
// export type RuleSetConditionAbsolute =
// 	| RegExp
// 	| String
// 	| ((value: String) => bool)
// 	| RuleSetConditionsAbsolute
// 	| {
// 			
// 			and: RuleSetConditionsAbsolute;
// 			
// 			exclude: RuleSetConditionOrConditionsAbsolute;
// 			
// 			include: RuleSetConditionOrConditionsAbsolute;
// 			
// 			not: RuleSetConditionsAbsolute;
// 			
// 			or: RuleSetConditionsAbsolute;
// 			
// 			test: RuleSetConditionOrConditionsAbsolute;
// 	  };
// via the `definition` "RuleSetConditionsAbsolute".
//  */
// export type RuleSetConditionsAbsolute = RuleSetConditionsAbsoluteRecursive;
// via the `definition` "RuleSetLoader".
//  */
// export type RuleSetLoader = String;
// via the `definition` "RuleSetUse".
//  */
// export type RuleSetUse = RuleSetUseItem | Function | RuleSetUseItem[];
// via the `definition` "RuleSetUseItem".
//  */
// export type RuleSetUseItem =
// 	| RuleSetLoader
// 	| Function
// 	| {
// 			
// 			ident: String;
// 			
// 			loader: RuleSetLoader;
// 			
// 			options: RuleSetQuery;
// 			
// 			query: RuleSetQuery;
// 	  };
// via the `definition` "RuleSetQuery".
//  */
// export type RuleSetQuery =
// 	| {
// 			[k: String]: any;
// 	  }
// 	| String;
// via the `definition` "ArrayOfStringOrStringArrayValues".
//  */
// export type ArrayOfStringOrStringArrayValues = (String | String[])[];
// 
//  * This interface was referenced by `WebpackOptions`'s JSON-Schema
//  * via the `definition` "WebpackPluginFunction".
//  */
// export type WebpackPluginFunction = (
// 	this: import("../lib/Compiler"),
// 	compiler: import("../lib/Compiler")
// ) => void;
// via the `definition` "RuleSetRules".
//  */
// export type RuleSetRules = RuleSetRule[];
// via the `definition` "FilterTypes".
//  */
// export type FilterTypes = FilterItemTypes | FilterItemTypes[];
// via the `definition` "FilterItemTypes".
//  */
// export type FilterItemTypes = RegExp | String | Function;

// const DEVELOPMENT: String = "development";
// const PRODUCTION: String = "production";
// const NONE: Stringing = "none".to_Stringing();


// enum Mode {
//     DEVELOPMENT,
//     PRODUCTION,
//     NONE
// }

#[derive(Serialize, Deserialize)]
pub struct RustpackOptions {
	
	// amd: {
	// 	[k: String]: any,
	// },
	
	pub bail: bool,
	
	pub cache: bool,
	
	//context: String,
	
	//dependencies: Vec<String>,
	
	// devServer: {
	// 	[k: String]: any,
	// },
	
	// devtool: String | false,
	
	// entry: Entry,
	
	// externals: Externals,
	
	// loader: {
	// 	[k: String]: any,
	// },
	
	//mode: String,
	
	// module: ModuleOptions,
	
	//name: String,
	
	// node: false | NodeOptions,
	// 
	// optimization: OptimizationOptions,
	// 
	// output: OutputOptions,
	// 
	// parallelism: number,
	
	// performance: false | PerformanceOptions,
	
	// plugins: (WebpackPluginInstance | WebpackPluginFunction)[],
	
	//profile: bool,
	
	// recordsInputPath: String,
	// 
	// recordsOutputPath: String,
	// 
	//recordsPath: String,
	
	// resolve: ResolveOptions,
	// 
	// resolveLoader: ResolveOptions,
	
	// serve: {
	// 	[k: String]: any,
	// },
	
	// stats:
	// 	| StatsOptions
	// 	| bool
	// 	| ("none" | "errors-only" | "minimal" | "normal" | "detailed" | "verbose"),
	// target:
	// 	| (
	// 			| "web"
	// 			| "webworker"
	// 			| "node"
	// 			| "async-node"
	// 			| "node-webkit"
	// 			| "electron-main"
	// 			| "electron-renderer")
	// 	| ((compiler: import("../lib/Compiler")) => void),
	//watch: bool,
	
	 //Options for the watcher
	 
	// watchOptions: {
	// 	
	// 	aggregateTimeout: number,
	// 	
	// 	ignored: {
	// 		[k: String]: any,
	// 	},
	// 	
	// 	poll: bool | number,
	// 	
	// 	stdin: bool,
	// },
}


// export interface EntryObject {
// 	
// 	[k: String]: String | NonEmptyArrayOfUniqueStringValues,
// }
// via the `definition` "ModuleOptions".
//  */
// export interface ModuleOptions {
// 	
// 	defaultRules: RuleSetRules,
// 	
// 	exprContextCritical: bool,
// 	
// 	exprContextRecursive: bool,
// 	
// 	exprContextRegExp: bool | RegExp,
// 	
// 	exprContextRequest: String,
// 	
// 	noParse: RegExp[] | RegExp | Function | String[] | String,
// 	
// 	rules: RuleSetRules,
// 	
// 	StringictExportPresence: bool,
// 	
// 	StringictThisContextOnImports: bool,
// 	
// 	unknownContextCritical: bool,
// 	
// 	unknownContextRecursive: bool,
// 	
// 	unknownContextRegExp: bool | RegExp,
// 	
// 	unknownContextRequest: String,
// 	
// 	unsafeCache: bool | Function,
// 	
// 	wrappedContextCritical: bool,
// 	
// 	wrappedContextRecursive: bool,
// 	
// 	wrappedContextRegExp: RegExp,
// }
// via the `definition` "RuleSetRule".
//  */
// export interface RuleSetRule {
// 	
// 	compiler: RuleSetConditionOrConditions,
// 	
// 	enforce: "pre" | "post",
// 	
// 	exclude: RuleSetConditionOrConditionsAbsolute,
// 	
// 	include: RuleSetConditionOrConditionsAbsolute,
// 	
// 	issuer: RuleSetConditionOrConditionsAbsolute,
// 	
// 	loader: RuleSetLoader | RuleSetUse,
// 	
// 	loaders: RuleSetUse,
// 	
// 	oneOf: RuleSetRules,
// 	
// 	options: RuleSetQuery,
// 	
// 	parser: {
// 		[k: String]: any,
// 	},
// 	
// 	query: RuleSetQuery,
// 	
// 	resolve: ResolveOptions,
// 	
// 	resource: RuleSetConditionOrConditionsAbsolute,
// 	
// 	resourceQuery: RuleSetConditionOrConditions,
// 	
// 	rules: RuleSetRules,
// 	
// 	sideEffects: bool,
// 	
// 	test: RuleSetConditionOrConditionsAbsolute,
// 	
// 	type:
// 		| "javascript/auto"
// 		| "javascript/dynamic"
// 		| "javascript/esm"
// 		| "json"
// 		| "webassembly/experimental",
// 	
// 	use: RuleSetUse,
// }
// via the `definition` "ResolveOptions".
//  */
// export interface ResolveOptions {
// 	
// 	alias:
// 		| {
// 				
// 				[k: String]: String,
// 		  }
// 		| {
// 				
// 				alias: String,
// 				
// 				name: String,
// 				
// 				onlyModule: bool,
// 		  }[],
// 	
// 	aliasFields: ArrayOfStringOrStringArrayValues,
// 	
// 	cachePredicate: Function,
// 	
// 	cacheWithContext: bool,
// 	
// 	concord: bool,
// 	
// 	descriptionFiles: ArrayOfStringValues,
// 	
// 	enforceExtension: bool,
// 	
// 	enforceModuleExtension: bool,
// 	
// 	extensions: ArrayOfStringValues,
// 	
// 	fileSystem: {
// 		[k: String]: any,
// 	},
// 	
// 	mainFields: ArrayOfStringOrStringArrayValues,
// 	
// 	mainFiles: ArrayOfStringValues,
// 	
// 	moduleExtensions: ArrayOfStringValues,
// 	
// 	modules: ArrayOfStringValues,
// 	
// 	plugins: (WebpackPluginInstance | WebpackPluginFunction)[],
// 	
// 	resolver: {
// 		[k: String]: any,
// 	},
// 	
// 	symlinks: bool,
// 	
// 	unsafeCache:
// 		| bool
// 		| {
// 				[k: String]: any,
// 		  },
// 	
// 	useSyncFileSystemCalls: bool,
// }
// 
//  * This interface was referenced by `WebpackOptions`'s JSON-Schema
//  * via the `definition` "WebpackPluginInstance".
//  */
// export interface WebpackPluginInstance {
// 	
// 	apply: (compiler: import("../lib/Compiler")) => void,
// 	[k: String]: any,
// }
// via the `definition` "NodeOptions".
//  */
// export interface NodeOptions {
// 	
// 	Buffer: false | true | "mock",
// 	
// 	__dirname: false | true | "mock",
// 	
// 	__filename: false | true | "mock",
// 	
// 	console: false | true | "mock",
// 	
// 	global: bool,
// 	
// 	process: false | true | "mock",
// 	
// 	[k: String]: false | true | "mock" | "empty",
// }
// 
//  * This interface was referenced by `WebpackOptions`'s JSON-Schema
//  * via the `definition` "OptimizationOptions".
//  */
// export interface OptimizationOptions {
// 	
// 	checkWasmTypes: bool,
// 	
// 	chunkIds: "natural" | "named" | "size" | "total-size" | false,
// 	
// 	concatenateModules: bool,
// 	
// 	flagIncludedChunks: bool,
// 	
// 	hashedModuleIds: bool,
// 	
// 	mangleWasmImports: bool,
// 	
// 	mergeDuplicateChunks: bool,
// 	
// 	minimize: bool,
// 	
// 	minimizer: (WebpackPluginInstance | WebpackPluginFunction)[],
// 	
// 	moduleIds: "natural" | "named" | "hashed" | "size" | "total-size" | false,
// 	
// 	namedChunks: bool,
// 	
// 	namedModules: bool,
// 	
// 	noEmitOnErrors: bool,
// 	
// 	nodeEnv: false | String,
// 	
// 	occurrenceOrder: bool,
// 	
// 	portableRecords: bool,
// 	
// 	providedExports: bool,
// 	
// 	removeAvailableModules: bool,
// 	
// 	removeEmptyChunks: bool,
// 	
// 	runtimeChunk:
// 		| bool
// 		| ("single" | "multiple")
// 		| {
// 				
// 				name: String | Function,
// 		  },
// 	
// 	sideEffects: bool,
// 	
// 	splitChunks: false | OptimizationSplitChunksOptions,
// 	
// 	usedExports: bool,
// }
// via the `definition` "OptimizationSplitChunksOptions".
//  */
// export interface OptimizationSplitChunksOptions {
// 	
// 	automaticNameDelimiter: String,
// 	
// 	cacheGroups: {
// 		
// 		[k: String]:
// 			| false
// 			| Function
// 			| String
// 			| RegExp
// 			| {
// 					
// 					automaticNameDelimiter: String,
// 					
// 					automaticNamePrefix: String,
// 					
// 					chunks: ("initial" | "async" | "all") | Function,
// 					
// 					enforce: bool,
// 					
// 					filename: String,
// 					
// 					maxAsyncRequests: number,
// 					
// 					maxInitialRequests: number,
// 					
// 					maxSize: number,
// 					
// 					minChunks: number,
// 					
// 					minSize: number,
// 					
// 					name: bool | Function | String,
// 					
// 					priority: number,
// 					
// 					reuseExistingChunk: bool,
// 					
// 					test: Function | String | RegExp,
// 			  },
// 	},
// 	
// 	chunks: ("initial" | "async" | "all") | Function,
// 	
// 	fallbackCacheGroup: {
// 		
// 		automaticNameDelimiter: String,
// 		
// 		maxSize: number,
// 		
// 		minSize: number,
// 	},
// 	
// 	filename: String,
// 	
// 	hidePathInfo: bool,
// 	
// 	maxAsyncRequests: number,
// 	
// 	maxInitialRequests: number,
// 	
// 	maxSize: number,
// 	
// 	minChunks: number,
// 	
// 	minSize: number,
// 	
// 	name: bool | Function | String,
// }
// via the `definition` "OutputOptions".
//  */
// export interface OutputOptions {
// 	
// 	auxiliaryComment:
// 		| String
// 		| {
// 				
// 				amd: String,
// 				
// 				commonjs: String,
// 				
// 				commonjs2: String,
// 				
// 				root: String,
// 		  },
// 	
// 	chunkCallbackName: String,
// 	
// 	chunkFilename: String,
// 	
// 	chunkLoadTimeout: number,
// 	
// 	crossOriginLoading: false | "anonymous" | "use-credentials",
// 	
// 	devtoolFallbackModuleFilenameTemplate: String | Function,
// 	
// 	devtoolLineToLine:
// 		| bool
// 		| {
// 				[k: String]: any,
// 		  },
// 	
// 	devtoolModuleFilenameTemplate: String | Function,
// 	
// 	devtoolNamespace: String,
// 	
// 	filename: String | Function,
// 	
// 	futureEmitAssets: bool,
// 	
// 	globalObject: String,
// 	
// 	hashDigest: String,
// 	
// 	hashDigestLength: number,
// 	
// 	hashFunction: String | (new () => import("../lib/util/createHash").Hash),
// 	
// 	hashSalt: String,
// 	
// 	hotUpdateChunkFilename: String | Function,
// 	
// 	hotUpdateFunction: String,
// 	
// 	hotUpdateMainFilename: String | Function,
// 	
// 	jsonpFunction: String,
// 	
// 	jsonpScriptType: false | "text/javascript" | "module",
// 	
// 	library: String | String[] | LibraryCustomUmdObject,
// 	
// 	libraryExport: String | ArrayOfStringValues,
// 	
// 	libraryTarget:
// 		| "var"
// 		| "assign"
// 		| "this"
// 		| "window"
// 		| "self"
// 		| "global"
// 		| "commonjs"
// 		| "commonjs2"
// 		| "commonjs-module"
// 		| "amd"
// 		| "amd-require"
// 		| "umd"
// 		| "umd2"
// 		| "jsonp",
// 	
// 	path: String,
// 	
// 	pathinfo: bool,
// 	
// 	publicPath: String | Function,
// 	
// 	sourceMapFilename: String,
// 	
// 	sourcePrefix: String,
// 	
// 	StringictModuleExceptionHandling: bool,
// 	
// 	umdNamedDefine: bool,
// 	
// 	webassemblyModuleFilename: String,
// }
// via the `definition` "LibraryCustomUmdObject".
//  */
// export interface LibraryCustomUmdObject {
// 	
// 	amd: String,
// 	
// 	commonjs: String,
// 	
// 	root: String | ArrayOfStringValues,
// }
// via the `definition` "PerformanceOptions".
//  */
// export interface PerformanceOptions {
// 	
// 	assetFilter: Function,
// 	
// 	hints: false | "warning" | "error",
// 	
// 	maxAssetSize: number,
// 	
// 	maxEntrypointSize: number,
// }
// via the `definition` "StatsOptions".
//  */
// export interface StatsOptions {
// 	
// 	all: bool,
// 	
// 	assets: bool,
// 	
// 	assetsSort: String,
// 	
// 	builtAt: bool,
// 	
// 	cached: bool,
// 	
// 	cachedAssets: bool,
// 	
// 	children: bool,
// 	
// 	chunkGroups: bool,
// 	
// 	chunkModules: bool,
// 	
// 	chunkOrigins: bool,
// 	
// 	chunks: bool,
// 	
// 	chunksSort: String,
// 	
// 	colors:
// 		| bool
// 		| {
// 				
// 				bold: String,
// 				
// 				cyan: String,
// 				
// 				green: String,
// 				
// 				magenta: String,
// 				
// 				red: String,
// 				
// 				yellow: String,
// 		  },
// 	
// 	context: String,
// 	
// 	depth: bool,
// 	
// 	entrypoints: bool,
// 	
// 	env: bool,
// 	
// 	errorDetails: bool,
// 	
// 	errors: bool,
// 	
// 	exclude: FilterTypes | bool,
// 	
// 	excludeAssets: FilterTypes,
// 	
// 	excludeModules: FilterTypes | bool,
// 	
// 	hash: bool,
// 	
// 	maxModules: number,
// 	
// 	moduleAssets: bool,
// 	
// 	moduleTrace: bool,
// 	
// 	modules: bool,
// 	
// 	modulesSort: String,
// 	
// 	nestedModules: bool,
// 	
// 	optimizationBailout: bool,
// 	
// 	outputPath: bool,
// 	
// 	performance: bool,
// 	
// 	providedExports: bool,
// 	
// 	publicPath: bool,
// 	
// 	reasons: bool,
// 	
// 	source: bool,
// 	
// 	timings: bool,
// 	
// 	usedExports: bool,
// 	
// 	version: bool,
// 	
// 	warnings: bool,
// 	
// 	warningsFilter: FilterTypes,
// }