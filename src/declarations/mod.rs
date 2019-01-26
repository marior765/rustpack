

extern crate futures;

use futures::*;

pub enum Entry {
    EntryDynamic, 
    EntryStatic
}

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

// /**
//  * This interface was referenced by `WebpackOptions`'s JSON-Schema
//  * via the `definition` "NonEmptyArrayOfUniqueStringValues".
//  */
// export type NonEmptyArrayOfUniqueStringValues = String[];
// /**
//  * This interface was referenced by `WebpackOptions`'s JSON-Schema
//  * via the `definition` "EntryItem".
//  */
// export type EntryItem = String | NonEmptyArrayOfUniqueStringValues;
// /**
//  * This interface was referenced by `WebpackOptions`'s JSON-Schema
//  * via the `definition` "Externals".
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
// /**
//  * This interface was referenced by `WebpackOptions`'s JSON-Schema
//  * via the `definition` "ExternalItem".
//  */
// export type ExternalItem =
// 	| String
// 	| {
// 			/**
// 			 * The dependency used for the external
// 			 */
// 			[k: String]:
// 				| String
// 				| {
// 						[k: String]: any;
// 				  }
// 				| ArrayOfStringValues
// 				| bool;
// 	  }
// 	| RegExp;
// /**
//  * This interface was referenced by `WebpackOptions`'s JSON-Schema
//  * via the `definition` "ArrayOfStringValues".
//  */
// export type ArrayOfStringValues = String[];
// /**
//  * One or multiple rule conditions
//  *
//  * This interface was referenced by `WebpackOptions`'s JSON-Schema
//  * via the `definition` "RuleSetConditionOrConditions".
//  */
// export type RuleSetConditionOrConditions = RuleSetCondition | RuleSetConditions;
// /**
//  * This interface was referenced by `WebpackOptions`'s JSON-Schema
//  * via the `definition` "RuleSetCondition".
//  */
// export type RuleSetCondition =
// 	| RegExp
// 	| String
// 	| ((value: String) => bool)
// 	| RuleSetConditions
// 	| {
// 			/**
// 			 * Logical AND
// 			 */
// 			and: RuleSetConditions;
// 			/**
// 			 * Exclude all modules matching any of these conditions
// 			 */
// 			exclude: RuleSetConditionOrConditions;
// 			/**
// 			 * Exclude all modules matching not any of these conditions
// 			 */
// 			include: RuleSetConditionOrConditions;
// 			/**
// 			 * Logical NOT
// 			 */
// 			not: RuleSetConditions;
// 			/**
// 			 * Logical OR
// 			 */
// 			or: RuleSetConditions;
// 			/**
// 			 * Exclude all modules matching any of these conditions
// 			 */
// 			test: RuleSetConditionOrConditions;
// 	  };
// /**
//  * This interface was referenced by `WebpackOptions`'s JSON-Schema
//  * via the `definition` "RuleSetConditions".
//  */
// export type RuleSetConditions = RuleSetConditionsRecursive;
// /**
//  * One or multiple rule conditions
//  *
//  * This interface was referenced by `WebpackOptions`'s JSON-Schema
//  * via the `definition` "RuleSetConditionOrConditionsAbsolute".
//  */
// export type RuleSetConditionOrConditionsAbsolute =
// 	| RuleSetConditionAbsolute
// 	| RuleSetConditionsAbsolute;
// /**
//  * This interface was referenced by `WebpackOptions`'s JSON-Schema
//  * via the `definition` "RuleSetConditionAbsolute".
//  */
// export type RuleSetConditionAbsolute =
// 	| RegExp
// 	| String
// 	| ((value: String) => bool)
// 	| RuleSetConditionsAbsolute
// 	| {
// 			/**
// 			 * Logical AND
// 			 */
// 			and: RuleSetConditionsAbsolute;
// 			/**
// 			 * Exclude all modules matching any of these conditions
// 			 */
// 			exclude: RuleSetConditionOrConditionsAbsolute;
// 			/**
// 			 * Exclude all modules matching not any of these conditions
// 			 */
// 			include: RuleSetConditionOrConditionsAbsolute;
// 			/**
// 			 * Logical NOT
// 			 */
// 			not: RuleSetConditionsAbsolute;
// 			/**
// 			 * Logical OR
// 			 */
// 			or: RuleSetConditionsAbsolute;
// 			/**
// 			 * Exclude all modules matching any of these conditions
// 			 */
// 			test: RuleSetConditionOrConditionsAbsolute;
// 	  };
// /**
//  * This interface was referenced by `WebpackOptions`'s JSON-Schema
//  * via the `definition` "RuleSetConditionsAbsolute".
//  */
// export type RuleSetConditionsAbsolute = RuleSetConditionsAbsoluteRecursive;
// /**
//  * This interface was referenced by `WebpackOptions`'s JSON-Schema
//  * via the `definition` "RuleSetLoader".
//  */
// export type RuleSetLoader = String;
// /**
//  * This interface was referenced by `WebpackOptions`'s JSON-Schema
//  * via the `definition` "RuleSetUse".
//  */
// export type RuleSetUse = RuleSetUseItem | Function | RuleSetUseItem[];
// /**
//  * This interface was referenced by `WebpackOptions`'s JSON-Schema
//  * via the `definition` "RuleSetUseItem".
//  */
// export type RuleSetUseItem =
// 	| RuleSetLoader
// 	| Function
// 	| {
// 			/**
// 			 * Unique loader identifier
// 			 */
// 			ident: String;
// 			/**
// 			 * Loader name
// 			 */
// 			loader: RuleSetLoader;
// 			/**
// 			 * Loader options
// 			 */
// 			options: RuleSetQuery;
// 			/**
// 			 * Loader query
// 			 */
// 			query: RuleSetQuery;
// 	  };
// /**
//  * This interface was referenced by `WebpackOptions`'s JSON-Schema
//  * via the `definition` "RuleSetQuery".
//  */
// export type RuleSetQuery =
// 	| {
// 			[k: String]: any;
// 	  }
// 	| String;
// /**
//  * This interface was referenced by `WebpackOptions`'s JSON-Schema
//  * via the `definition` "ArrayOfStringOrStringArrayValues".
//  */
// export type ArrayOfStringOrStringArrayValues = (String | String[])[];
// /**
//  * Function acting as plugin
//  *
//  * This interface was referenced by `WebpackOptions`'s JSON-Schema
//  * via the `definition` "WebpackPluginFunction".
//  */
// export type WebpackPluginFunction = (
// 	this: import("../lib/Compiler"),
// 	compiler: import("../lib/Compiler")
// ) => void;
// /**
//  * This interface was referenced by `WebpackOptions`'s JSON-Schema
//  * via the `definition` "RuleSetRules".
//  */
// export type RuleSetRules = RuleSetRule[];
// /**
//  * This interface was referenced by `WebpackOptions`'s JSON-Schema
//  * via the `definition` "FilterTypes".
//  */
// export type FilterTypes = FilterItemTypes | FilterItemTypes[];
// /**
//  * This interface was referenced by `WebpackOptions`'s JSON-Schema
//  * via the `definition` "FilterItemTypes".
//  */
// export type FilterItemTypes = RegExp | String | Function;

// const DEVELOPMENT: String = "development";
// const PRODUCTION: String = "production";
// const NONE: Stringing = "none".to_Stringing();


enum Mode {
    DEVELOPMENT,
    PRODUCTION,
    NONE
}

pub struct RustpackOptions {
	/**
	 * Set the value of `require.amd` and `define.amd`.
	 */
	// amd: {
	// 	[k: String]: any,
	// },
	/**
	 * Report the first error as a hard error instead of tolerating it.
	 */
	bail: bool,
	/**
	 * Cache generated modules and chunks to improve performance for multiple incremental builds.
	 */
	cache: bool,
	/**
	 * The base directory (absolute path!) for resolving the `entry` option. If `output.pathinfo` is set, the included pathinfo is shortened to this directory.
	 */
	context: String,
	/**
	 * References to other configurations to depend on.
	 */
	dependencies: Vec<String>,
	/**
	 * Options for the webpack-dev-server
	 */
	// devServer: {
	// 	[k: String]: any,
	// },
	/**
	 * A developer tool to enhance debugging.
	 */
	// devtool: String | false,
	/**
	 * The entry point(s) of the compilation.
	 */
	// entry: Entry,
	/**
	 * Specify dependencies that shouldn't be resolved by webpack, but should become dependencies of the resulting bundle. The kind of the dependency depends on `output.libraryTarget`.
	 */
	// externals: Externals,
	/**
	 * Custom values available in the loader context.
	 */
	// loader: {
	// 	[k: String]: any,
	// },
	/**
	 * Enable production optimizations or development hints.
	 */
	mode: String,
	/**
	 * Options affecting the normal modules (`NormalModuleFactory`).
	 */
	// module: ModuleOptions,
	/**
	 * Name of the configuration. Used when loading multiple configurations.
	 */
	name: String,
	/**
	 * Include polyfills or mocks for various node stuff.
	 */
	// node: false | NodeOptions,
	// /**
	//  * Enables/Disables integrated optimizations
	//  */
	// optimization: OptimizationOptions,
	// /**
	//  * Options affecting the output of the compilation. `output` options tell webpack how to write the compiled files to disk.
	//  */
	// output: OutputOptions,
	// /**
	//  * The number of parallel processed modules in the compilation.
	//  */
	// parallelism: number,
	/**
	 * Configuration for web performance recommendations.
	 */
	// performance: false | PerformanceOptions,
	/**
	 * Add additional plugins to the compiler.
	 */
	// plugins: (WebpackPluginInstance | WebpackPluginFunction)[],
	/**
	 * Capture timing information for each module.
	 */
	profile: bool,
	/**
	 * Store compiler state to a json file.
	 */
	// recordsInputPath: String,
	// /**
	//  * Load compiler state from a json file.
	//  */
	// recordsOutputPath: String,
	// /**
	//  * Store/Load compiler state from/to a json file. This will result in persistent ids of modules and chunks. An absolute path is expected. `recordsPath` is used for `recordsInputPath` and `recordsOutputPath` if they left undefined.
	//  */
	recordsPath: String,
	/**
	 * Options for the resolver
	 */
	// resolve: ResolveOptions,
	// /**
	//  * Options for the resolver when resolving loaders
	//  */
	// resolveLoader: ResolveOptions,
	/**
	 * Options for webpack-serve
	 */
	// serve: {
	// 	[k: String]: any,
	// },
	/**
	 * Used by the webpack CLI program to pass stats options.
	 */
	// stats:
	// 	| StatsOptions
	// 	| bool
	// 	| ("none" | "errors-only" | "minimal" | "normal" | "detailed" | "verbose"),
	/**
	 * Environment to build for
	 */
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
	/**
	 * Enter watch mode, which rebuilds on file change.
	 */
	watch: bool,
	
	 //Options for the watcher
	 
	// watchOptions: {
	// 	/**
	// 	 * Delay the rebuilt after the first change. Value is a time in ms.
	// 	 */
	// 	aggregateTimeout: number,
	// 	/**
	// 	 * Ignore some files from watching
	// 	 */
	// 	ignored: {
	// 		[k: String]: any,
	// 	},
	// 	/**
	// 	 * Enable polling mode for watching
	// 	 */
	// 	poll: bool | number,
	// 	/**
	// 	 * Stop watching when stdin Stringeam has ended
	// 	 */
	// 	stdin: bool,
	// },
}


// export interface EntryObject {
// 	/**
// 	 * An entry point with name
// 	 */
// 	[k: String]: String | NonEmptyArrayOfUniqueStringValues,
// }
// /**
//  * This interface was referenced by `WebpackOptions`'s JSON-Schema
//  * via the `definition` "ModuleOptions".
//  */
// export interface ModuleOptions {
// 	/**
// 	 * An array of rules applied by default for modules.
// 	 */
// 	defaultRules: RuleSetRules,
// 	/**
// 	 * Enable warnings for full dynamic dependencies
// 	 */
// 	exprContextCritical: bool,
// 	/**
// 	 * Enable recursive directory lookup for full dynamic dependencies
// 	 */
// 	exprContextRecursive: bool,
// 	/**
// 	 * Sets the default regular expression for full dynamic dependencies
// 	 */
// 	exprContextRegExp: bool | RegExp,
// 	/**
// 	 * Set the default request for full dynamic dependencies
// 	 */
// 	exprContextRequest: String,
// 	/**
// 	 * Don't parse files matching. It's matched against the full resolved request.
// 	 */
// 	noParse: RegExp[] | RegExp | Function | String[] | String,
// 	/**
// 	 * An array of rules applied for modules.
// 	 */
// 	rules: RuleSetRules,
// 	/**
// 	 * Emit errors instead of warnings when imported names don't exist in imported module
// 	 */
// 	StringictExportPresence: bool,
// 	/**
// 	 * Handle the this context correctly according to the spec for namespace objects
// 	 */
// 	StringictThisContextOnImports: bool,
// 	/**
// 	 * Enable warnings when using the require function in a not statically analyse-able way
// 	 */
// 	unknownContextCritical: bool,
// 	/**
// 	 * Enable recursive directory lookup when using the require function in a not statically analyse-able way
// 	 */
// 	unknownContextRecursive: bool,
// 	/**
// 	 * Sets the regular expression when using the require function in a not statically analyse-able way
// 	 */
// 	unknownContextRegExp: bool | RegExp,
// 	/**
// 	 * Sets the request when using the require function in a not statically analyse-able way
// 	 */
// 	unknownContextRequest: String,
// 	/**
// 	 * Cache the resolving of module requests
// 	 */
// 	unsafeCache: bool | Function,
// 	/**
// 	 * Enable warnings for partial dynamic dependencies
// 	 */
// 	wrappedContextCritical: bool,
// 	/**
// 	 * Enable recursive directory lookup for partial dynamic dependencies
// 	 */
// 	wrappedContextRecursive: bool,
// 	/**
// 	 * Set the inner regular expression for partial dynamic dependencies
// 	 */
// 	wrappedContextRegExp: RegExp,
// }
// /**
//  * This interface was referenced by `WebpackOptions`'s JSON-Schema
//  * via the `definition` "RuleSetRule".
//  */
// export interface RuleSetRule {
// 	/**
// 	 * Match the child compiler name
// 	 */
// 	compiler: RuleSetConditionOrConditions,
// 	/**
// 	 * Enforce this rule as pre or post step
// 	 */
// 	enforce: "pre" | "post",
// 	/**
// 	 * Shortcut for resource.exclude
// 	 */
// 	exclude: RuleSetConditionOrConditionsAbsolute,
// 	/**
// 	 * Shortcut for resource.include
// 	 */
// 	include: RuleSetConditionOrConditionsAbsolute,
// 	/**
// 	 * Match the issuer of the module (The module pointing to this module)
// 	 */
// 	issuer: RuleSetConditionOrConditionsAbsolute,
// 	/**
// 	 * Shortcut for use.loader
// 	 */
// 	loader: RuleSetLoader | RuleSetUse,
// 	/**
// 	 * Shortcut for use.loader
// 	 */
// 	loaders: RuleSetUse,
// 	/**
// 	 * Only execute the first matching rule in this array
// 	 */
// 	oneOf: RuleSetRules,
// 	/**
// 	 * Shortcut for use.options
// 	 */
// 	options: RuleSetQuery,
// 	/**
// 	 * Options for parsing
// 	 */
// 	parser: {
// 		[k: String]: any,
// 	},
// 	/**
// 	 * Shortcut for use.query
// 	 */
// 	query: RuleSetQuery,
// 	/**
// 	 * Options for the resolver
// 	 */
// 	resolve: ResolveOptions,
// 	/**
// 	 * Match the resource path of the module
// 	 */
// 	resource: RuleSetConditionOrConditionsAbsolute,
// 	/**
// 	 * Match the resource query of the module
// 	 */
// 	resourceQuery: RuleSetConditionOrConditions,
// 	/**
// 	 * Match and execute these rules when this rule is matched
// 	 */
// 	rules: RuleSetRules,
// 	/**
// 	 * Flags a module as with or without side effects
// 	 */
// 	sideEffects: bool,
// 	/**
// 	 * Shortcut for resource.test
// 	 */
// 	test: RuleSetConditionOrConditionsAbsolute,
// 	/**
// 	 * Module type to use for the module
// 	 */
// 	type:
// 		| "javascript/auto"
// 		| "javascript/dynamic"
// 		| "javascript/esm"
// 		| "json"
// 		| "webassembly/experimental",
// 	/**
// 	 * Modifiers applied to the module when rule is matched
// 	 */
// 	use: RuleSetUse,
// }
// /**
//  * This interface was referenced by `WebpackOptions`'s JSON-Schema
//  * via the `definition` "ResolveOptions".
//  */
// export interface ResolveOptions {
// 	/**
// 	 * Redirect module requests
// 	 */
// 	alias:
// 		| {
// 				/**
// 				 * New request
// 				 */
// 				[k: String]: String,
// 		  }
// 		| {
// 				/**
// 				 * New request
// 				 */
// 				alias: String,
// 				/**
// 				 * Request to be redirected
// 				 */
// 				name: String,
// 				/**
// 				 * Redirect only exact matching request
// 				 */
// 				onlyModule: bool,
// 		  }[],
// 	/**
// 	 * Fields in the description file (package.json) which are used to redirect requests inside the module
// 	 */
// 	aliasFields: ArrayOfStringOrStringArrayValues,
// 	/**
// 	 * Predicate function to decide which requests should be cached
// 	 */
// 	cachePredicate: Function,
// 	/**
// 	 * Include the context information in the cache identifier when caching
// 	 */
// 	cacheWithContext: bool,
// 	/**
// 	 * Enable concord resolving extras
// 	 */
// 	concord: bool,
// 	/**
// 	 * Filenames used to find a description file
// 	 */
// 	descriptionFiles: ArrayOfStringValues,
// 	/**
// 	 * Enforce using one of the extensions from the extensions option
// 	 */
// 	enforceExtension: bool,
// 	/**
// 	 * Enforce using one of the module extensions from the moduleExtensions option
// 	 */
// 	enforceModuleExtension: bool,
// 	/**
// 	 * Extensions added to the request when trying to find the file
// 	 */
// 	extensions: ArrayOfStringValues,
// 	/**
// 	 * Filesystem for the resolver
// 	 */
// 	fileSystem: {
// 		[k: String]: any,
// 	},
// 	/**
// 	 * Field names from the description file (package.json) which are used to find the default entry point
// 	 */
// 	mainFields: ArrayOfStringOrStringArrayValues,
// 	/**
// 	 * Filenames used to find the default entry point if there is no description file or main field
// 	 */
// 	mainFiles: ArrayOfStringValues,
// 	/**
// 	 * Extensions added to the module request when trying to find the module
// 	 */
// 	moduleExtensions: ArrayOfStringValues,
// 	/**
// 	 * Folder names or directory paths where to find modules
// 	 */
// 	modules: ArrayOfStringValues,
// 	/**
// 	 * Plugins for the resolver
// 	 */
// 	plugins: (WebpackPluginInstance | WebpackPluginFunction)[],
// 	/**
// 	 * Custom resolver
// 	 */
// 	resolver: {
// 		[k: String]: any,
// 	},
// 	/**
// 	 * Enable resolving symlinks to the original location
// 	 */
// 	symlinks: bool,
// 	/**
// 	 * Enable caching of successfully resolved requests
// 	 */
// 	unsafeCache:
// 		| bool
// 		| {
// 				[k: String]: any,
// 		  },
// 	/**
// 	 * Use synchronous filesystem calls for the resolver
// 	 */
// 	useSyncFileSystemCalls: bool,
// }
// /**
//  * Plugin instance
//  *
//  * This interface was referenced by `WebpackOptions`'s JSON-Schema
//  * via the `definition` "WebpackPluginInstance".
//  */
// export interface WebpackPluginInstance {
// 	/**
// 	 * The run point of the plugin, required method.
// 	 */
// 	apply: (compiler: import("../lib/Compiler")) => void,
// 	[k: String]: any,
// }
// /**
//  * This interface was referenced by `WebpackOptions`'s JSON-Schema
//  * via the `definition` "NodeOptions".
//  */
// export interface NodeOptions {
// 	/**
// 	 * Include a polyfill for the 'Buffer' variable
// 	 */
// 	Buffer: false | true | "mock",
// 	/**
// 	 * Include a polyfill for the '__dirname' variable
// 	 */
// 	__dirname: false | true | "mock",
// 	/**
// 	 * Include a polyfill for the '__filename' variable
// 	 */
// 	__filename: false | true | "mock",
// 	/**
// 	 * Include a polyfill for the 'console' variable
// 	 */
// 	console: false | true | "mock",
// 	/**
// 	 * Include a polyfill for the 'global' variable
// 	 */
// 	global: bool,
// 	/**
// 	 * Include a polyfill for the 'process' variable
// 	 */
// 	process: false | true | "mock",
// 	/**
// 	 * Include a polyfill for the node.js module
// 	 */
// 	[k: String]: false | true | "mock" | "empty",
// }
// /**
//  * Enables/Disables integrated optimizations
//  *
//  * This interface was referenced by `WebpackOptions`'s JSON-Schema
//  * via the `definition` "OptimizationOptions".
//  */
// export interface OptimizationOptions {
// 	/**
// 	 * Check for incompatible wasm types when importing/exporting from/to ESM
// 	 */
// 	checkWasmTypes: bool,
// 	/**
// 	 * Define the algorithm to choose chunk ids (named: readable ids for better debugging, size: numeric ids focused on minimal initial download size, total-size: numeric ids focused on minimal total download size, false: no algorithm used, as custom one can be provided via plugin)
// 	 */
// 	chunkIds: "natural" | "named" | "size" | "total-size" | false,
// 	/**
// 	 * Concatenate modules when possible to generate less modules, more efficient code and enable more optimizations by the minimizer
// 	 */
// 	concatenateModules: bool,
// 	/**
// 	 * Also flag chunks as loaded which contain a subset of the modules
// 	 */
// 	flagIncludedChunks: bool,
// 	/**
// 	 * Use hashed module id instead module identifiers for better long term caching (deprecated, used moduleIds: hashed instead)
// 	 */
// 	hashedModuleIds: bool,
// 	/**
// 	 * Reduce size of WASM by changing imports to shorter Strings.
// 	 */
// 	mangleWasmImports: bool,
// 	/**
// 	 * Merge chunks which contain the same modules
// 	 */
// 	mergeDuplicateChunks: bool,
// 	/**
// 	 * Enable minimizing the output. Uses optimization.minimizer.
// 	 */
// 	minimize: bool,
// 	/**
// 	 * Minimizer(s) to use for minimizing the output
// 	 */
// 	minimizer: (WebpackPluginInstance | WebpackPluginFunction)[],
// 	/**
// 	 * Define the algorithm to choose module ids (natural: numeric ids in order of usage, named: readable ids for better debugging, hashed: short hashes as ids for better long term caching, size: numeric ids focused on minimal initial download size, total-size: numeric ids focused on minimal total download size, false: no algorithm used, as custom one can be provided via plugin)
// 	 */
// 	moduleIds: "natural" | "named" | "hashed" | "size" | "total-size" | false,
// 	/**
// 	 * Use readable chunk identifiers for better debugging (deprecated, used chunkIds: named instead)
// 	 */
// 	namedChunks: bool,
// 	/**
// 	 * Use readable module identifiers for better debugging (deprecated, used moduleIds: named instead)
// 	 */
// 	namedModules: bool,
// 	/**
// 	 * Avoid emitting assets when errors occur
// 	 */
// 	noEmitOnErrors: bool,
// 	/**
// 	 * Set process.env.NODE_ENV to a specific value
// 	 */
// 	nodeEnv: false | String,
// 	/**
// 	 * Figure out a order of modules which results in the smallest initial bundle
// 	 */
// 	occurrenceOrder: bool,
// 	/**
// 	 * Generate records with relative paths to be able to move the context folder
// 	 */
// 	portableRecords: bool,
// 	/**
// 	 * Figure out which exports are provided by modules to generate more efficient code
// 	 */
// 	providedExports: bool,
// 	/**
// 	 * Removes modules from chunks when these modules are already included in all parents
// 	 */
// 	removeAvailableModules: bool,
// 	/**
// 	 * Remove chunks which are empty
// 	 */
// 	removeEmptyChunks: bool,
// 	/**
// 	 * Create an additional chunk which contains only the webpack runtime and chunk hash maps
// 	 */
// 	runtimeChunk:
// 		| bool
// 		| ("single" | "multiple")
// 		| {
// 				/**
// 				 * The name or name factory for the runtime chunks
// 				 */
// 				name: String | Function,
// 		  },
// 	/**
// 	 * Skip over modules which are flagged to contain no side effects when exports are not used
// 	 */
// 	sideEffects: bool,
// 	/**
// 	 * Optimize duplication and caching by splitting chunks by shared modules and cache group
// 	 */
// 	splitChunks: false | OptimizationSplitChunksOptions,
// 	/**
// 	 * Figure out which exports are used by modules to mangle export names, omit unused exports and generate more efficient code
// 	 */
// 	usedExports: bool,
// }
// /**
//  * This interface was referenced by `WebpackOptions`'s JSON-Schema
//  * via the `definition` "OptimizationSplitChunksOptions".
//  */
// export interface OptimizationSplitChunksOptions {
// 	/**
// 	 * Sets the name delimiter for created chunks
// 	 */
// 	automaticNameDelimiter: String,
// 	/**
// 	 * Assign modules to a cache group (modules from different cache groups are tried to keep in separate chunks)
// 	 */
// 	cacheGroups: {
// 		/**
// 		 * Configuration for a cache group
// 		 */
// 		[k: String]:
// 			| false
// 			| Function
// 			| String
// 			| RegExp
// 			| {
// 					/**
// 					 * Sets the name delimiter for created chunks
// 					 */
// 					automaticNameDelimiter: String,
// 					/**
// 					 * Sets the name prefix for created chunks
// 					 */
// 					automaticNamePrefix: String,
// 					/**
// 					 * Select chunks for determining cache group content (defaults to "initial", "initial" and "all" requires adding these chunks to the HTML)
// 					 */
// 					chunks: ("initial" | "async" | "all") | Function,
// 					/**
// 					 * Ignore minimum size, minimum chunks and maximum requests and always create chunks for this cache group
// 					 */
// 					enforce: bool,
// 					/**
// 					 * Sets the template for the filename for created chunks (Only works for initial chunks)
// 					 */
// 					filename: String,
// 					/**
// 					 * Maximum number of requests which are accepted for on-demand loading
// 					 */
// 					maxAsyncRequests: number,
// 					/**
// 					 * Maximum number of initial chunks which are accepted for an entry point
// 					 */
// 					maxInitialRequests: number,
// 					/**
// 					 * Maximal size hint for the created chunks
// 					 */
// 					maxSize: number,
// 					/**
// 					 * Minimum number of times a module has to be duplicated until it's considered for splitting
// 					 */
// 					minChunks: number,
// 					/**
// 					 * Minimal size for the created chunk
// 					 */
// 					minSize: number,
// 					/**
// 					 * Give chunks for this cache group a name (chunks with equal name are merged)
// 					 */
// 					name: bool | Function | String,
// 					/**
// 					 * Priority of this cache group
// 					 */
// 					priority: number,
// 					/**
// 					 * Try to reuse existing chunk (with name) when it has matching modules
// 					 */
// 					reuseExistingChunk: bool,
// 					/**
// 					 * Assign modules to a cache group
// 					 */
// 					test: Function | String | RegExp,
// 			  },
// 	},
// 	/**
// 	 * Select chunks for determining shared modules (defaults to "async", "initial" and "all" requires adding these chunks to the HTML)
// 	 */
// 	chunks: ("initial" | "async" | "all") | Function,
// 	/**
// 	 * Options for modules not selected by any other cache group
// 	 */
// 	fallbackCacheGroup: {
// 		/**
// 		 * Sets the name delimiter for created chunks
// 		 */
// 		automaticNameDelimiter: String,
// 		/**
// 		 * Maximal size hint for the created chunks
// 		 */
// 		maxSize: number,
// 		/**
// 		 * Minimal size for the created chunk
// 		 */
// 		minSize: number,
// 	},
// 	/**
// 	 * Sets the template for the filename for created chunks (Only works for initial chunks)
// 	 */
// 	filename: String,
// 	/**
// 	 * Prevents exposing path info when creating names for parts splitted by maxSize
// 	 */
// 	hidePathInfo: bool,
// 	/**
// 	 * Maximum number of requests which are accepted for on-demand loading
// 	 */
// 	maxAsyncRequests: number,
// 	/**
// 	 * Maximum number of initial chunks which are accepted for an entry point
// 	 */
// 	maxInitialRequests: number,
// 	/**
// 	 * Maximal size hint for the created chunks
// 	 */
// 	maxSize: number,
// 	/**
// 	 * Minimum number of times a module has to be duplicated until it's considered for splitting
// 	 */
// 	minChunks: number,
// 	/**
// 	 * Minimal size for the created chunks
// 	 */
// 	minSize: number,
// 	/**
// 	 * Give chunks created a name (chunks with equal name are merged)
// 	 */
// 	name: bool | Function | String,
// }
// /**
//  * This interface was referenced by `WebpackOptions`'s JSON-Schema
//  * via the `definition` "OutputOptions".
//  */
// export interface OutputOptions {
// 	/**
// 	 * Add a comment in the UMD wrapper.
// 	 */
// 	auxiliaryComment:
// 		| String
// 		| {
// 				/**
// 				 * Set comment for `amd` section in UMD
// 				 */
// 				amd: String,
// 				/**
// 				 * Set comment for `commonjs` (exports) section in UMD
// 				 */
// 				commonjs: String,
// 				/**
// 				 * Set comment for `commonjs2` (module.exports) section in UMD
// 				 */
// 				commonjs2: String,
// 				/**
// 				 * Set comment for `root` (global variable) section in UMD
// 				 */
// 				root: String,
// 		  },
// 	/**
// 	 * The callback function name used by webpack for loading of chunks in WebWorkers.
// 	 */
// 	chunkCallbackName: String,
// 	/**
// 	 * The filename of non-entry chunks as relative path inside the `output.path` directory.
// 	 */
// 	chunkFilename: String,
// 	/**
// 	 * Number of milliseconds before chunk request expires
// 	 */
// 	chunkLoadTimeout: number,
// 	/**
// 	 * This option enables cross-origin loading of chunks.
// 	 */
// 	crossOriginLoading: false | "anonymous" | "use-credentials",
// 	/**
// 	 * Similar to `output.devtoolModuleFilenameTemplate`, but used in the case of duplicate module identifiers.
// 	 */
// 	devtoolFallbackModuleFilenameTemplate: String | Function,
// 	/**
// 	 * Enable line to line mapped mode for all/specified modules. Line to line mapped mode uses a simple SourceMap where each line of the generated source is mapped to the same line of the original source. It’s a performance optimization. Only use it if your performance need to be better and you are sure that input lines match which generated lines.
// 	 */
// 	devtoolLineToLine:
// 		| bool
// 		| {
// 				[k: String]: any,
// 		  },
// 	/**
// 	 * Filename template String of function for the sources array in a generated SourceMap.
// 	 */
// 	devtoolModuleFilenameTemplate: String | Function,
// 	/**
// 	 * Module namespace to use when interpolating filename template String for the sources array in a generated SourceMap. Defaults to `output.library` if not set. It's useful for avoiding runtime collisions in sourcemaps from multiple webpack projects built as libraries.
// 	 */
// 	devtoolNamespace: String,
// 	/**
// 	 * Specifies the name of each output file on disk. You must **not** specify an absolute path here! The `output.path` option determines the location on disk the files are written to, filename is used solely for naming the individual files.
// 	 */
// 	filename: String | Function,
// 	/**
// 	 * Use the future version of asset emitting logic, which is allows freeing memory of assets after emitting. It could break plugins which assume that assets are still readable after emitting. Will be the new default in the next major version.
// 	 */
// 	futureEmitAssets: bool,
// 	/**
// 	 * An expression which is used to address the global object/scope in runtime code
// 	 */
// 	globalObject: String,
// 	/**
// 	 * Digest type used for the hash
// 	 */
// 	hashDigest: String,
// 	/**
// 	 * Number of chars which are used for the hash
// 	 */
// 	hashDigestLength: number,
// 	/**
// 	 * Algorithm used for generation the hash (see node.js crypto package)
// 	 */
// 	hashFunction: String | (new () => import("../lib/util/createHash").Hash),
// 	/**
// 	 * Any String which is added to the hash to salt it
// 	 */
// 	hashSalt: String,
// 	/**
// 	 * The filename of the Hot Update Chunks. They are inside the output.path directory.
// 	 */
// 	hotUpdateChunkFilename: String | Function,
// 	/**
// 	 * The JSONP function used by webpack for async loading of hot update chunks.
// 	 */
// 	hotUpdateFunction: String,
// 	/**
// 	 * The filename of the Hot Update Main File. It is inside the `output.path` directory.
// 	 */
// 	hotUpdateMainFilename: String | Function,
// 	/**
// 	 * The JSONP function used by webpack for async loading of chunks.
// 	 */
// 	jsonpFunction: String,
// 	/**
// 	 * This option enables loading async chunks via a custom script type, such as script type="module"
// 	 */
// 	jsonpScriptType: false | "text/javascript" | "module",
// 	/**
// 	 * If set, export the bundle as library. `output.library` is the name.
// 	 */
// 	library: String | String[] | LibraryCustomUmdObject,
// 	/**
// 	 * Specify which export should be exposed as library
// 	 */
// 	libraryExport: String | ArrayOfStringValues,
// 	/**
// 	 * Type of library
// 	 */
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
// 	/**
// 	 * The output directory as **absolute path** (required).
// 	 */
// 	path: String,
// 	/**
// 	 * Include comments with information about the modules.
// 	 */
// 	pathinfo: bool,
// 	/**
// 	 * The `publicPath` specifies the public URL address of the output files when referenced in a browser.
// 	 */
// 	publicPath: String | Function,
// 	/**
// 	 * The filename of the SourceMaps for the JavaScript files. They are inside the `output.path` directory.
// 	 */
// 	sourceMapFilename: String,
// 	/**
// 	 * Prefixes every line of the source in the bundle with this String.
// 	 */
// 	sourcePrefix: String,
// 	/**
// 	 * Handles exceptions in module loading correctly at a performance cost.
// 	 */
// 	StringictModuleExceptionHandling: bool,
// 	/**
// 	 * If `output.libraryTarget` is set to umd and `output.library` is set, setting this to true will name the AMD module.
// 	 */
// 	umdNamedDefine: bool,
// 	/**
// 	 * The filename of WebAssembly modules as relative path inside the `output.path` directory.
// 	 */
// 	webassemblyModuleFilename: String,
// }
// /**
//  * This interface was referenced by `WebpackOptions`'s JSON-Schema
//  * via the `definition` "LibraryCustomUmdObject".
//  */
// export interface LibraryCustomUmdObject {
// 	/**
// 	 * Name of the exposed AMD library in the UMD
// 	 */
// 	amd: String,
// 	/**
// 	 * Name of the exposed commonjs export in the UMD
// 	 */
// 	commonjs: String,
// 	/**
// 	 * Name of the property exposed globally by a UMD library
// 	 */
// 	root: String | ArrayOfStringValues,
// }
// /**
//  * This interface was referenced by `WebpackOptions`'s JSON-Schema
//  * via the `definition` "PerformanceOptions".
//  */
// export interface PerformanceOptions {
// 	/**
// 	 * Filter function to select assets that are checked
// 	 */
// 	assetFilter: Function,
// 	/**
// 	 * Sets the format of the hints: warnings, errors or nothing at all
// 	 */
// 	hints: false | "warning" | "error",
// 	/**
// 	 * Filesize limit (in bytes) when exceeded, that webpack will provide performance hints
// 	 */
// 	maxAssetSize: number,
// 	/**
// 	 * Total size of an entry point (in bytes)
// 	 */
// 	maxEntrypointSize: number,
// }
// /**
//  * This interface was referenced by `WebpackOptions`'s JSON-Schema
//  * via the `definition` "StatsOptions".
//  */
// export interface StatsOptions {
// 	/**
// 	 * fallback value for stats options when an option is not defined (has precedence over local webpack defaults)
// 	 */
// 	all: bool,
// 	/**
// 	 * add assets information
// 	 */
// 	assets: bool,
// 	/**
// 	 * sort the assets by that field
// 	 */
// 	assetsSort: String,
// 	/**
// 	 * add built at time information
// 	 */
// 	builtAt: bool,
// 	/**
// 	 * add also information about cached (not built) modules
// 	 */
// 	cached: bool,
// 	/**
// 	 * Show cached assets (setting this to `false` only shows emitted files)
// 	 */
// 	cachedAssets: bool,
// 	/**
// 	 * add children information
// 	 */
// 	children: bool,
// 	/**
// 	 * Display all chunk groups with the corresponding bundles
// 	 */
// 	chunkGroups: bool,
// 	/**
// 	 * add built modules information to chunk information
// 	 */
// 	chunkModules: bool,
// 	/**
// 	 * add the origins of chunks and chunk merging info
// 	 */
// 	chunkOrigins: bool,
// 	/**
// 	 * add chunk information
// 	 */
// 	chunks: bool,
// 	/**
// 	 * sort the chunks by that field
// 	 */
// 	chunksSort: String,
// 	/**
// 	 * Enables/Disables colorful output
// 	 */
// 	colors:
// 		| bool
// 		| {
// 				/**
// 				 * Custom color for bold text
// 				 */
// 				bold: String,
// 				/**
// 				 * Custom color for cyan text
// 				 */
// 				cyan: String,
// 				/**
// 				 * Custom color for green text
// 				 */
// 				green: String,
// 				/**
// 				 * Custom color for magenta text
// 				 */
// 				magenta: String,
// 				/**
// 				 * Custom color for red text
// 				 */
// 				red: String,
// 				/**
// 				 * Custom color for yellow text
// 				 */
// 				yellow: String,
// 		  },
// 	/**
// 	 * context directory for request shortening
// 	 */
// 	context: String,
// 	/**
// 	 * add module depth in module graph
// 	 */
// 	depth: bool,
// 	/**
// 	 * Display the entry points with the corresponding bundles
// 	 */
// 	entrypoints: bool,
// 	/**
// 	 * add --env information
// 	 */
// 	env: bool,
// 	/**
// 	 * add details to errors (like resolving log)
// 	 */
// 	errorDetails: bool,
// 	/**
// 	 * add errors
// 	 */
// 	errors: bool,
// 	/**
// 	 * Please use excludeModules instead.
// 	 */
// 	exclude: FilterTypes | bool,
// 	/**
// 	 * Suppress assets that match the specified filters. Filters can be Strings, RegExps or Functions
// 	 */
// 	excludeAssets: FilterTypes,
// 	/**
// 	 * Suppress modules that match the specified filters. Filters can be Strings, RegExps, bools or Functions
// 	 */
// 	excludeModules: FilterTypes | bool,
// 	/**
// 	 * add the hash of the compilation
// 	 */
// 	hash: bool,
// 	/**
// 	 * Set the maximum number of modules to be shown
// 	 */
// 	maxModules: number,
// 	/**
// 	 * add information about assets inside modules
// 	 */
// 	moduleAssets: bool,
// 	/**
// 	 * add dependencies and origin of warnings/errors
// 	 */
// 	moduleTrace: bool,
// 	/**
// 	 * add built modules information
// 	 */
// 	modules: bool,
// 	/**
// 	 * sort the modules by that field
// 	 */
// 	modulesSort: String,
// 	/**
// 	 * add information about modules nested in other modules (like with module concatenation)
// 	 */
// 	nestedModules: bool,
// 	/**
// 	 * show reasons why optimization bailed out for modules
// 	 */
// 	optimizationBailout: bool,
// 	/**
// 	 * Add output path information
// 	 */
// 	outputPath: bool,
// 	/**
// 	 * add performance hint flags
// 	 */
// 	performance: bool,
// 	/**
// 	 * show exports provided by modules
// 	 */
// 	providedExports: bool,
// 	/**
// 	 * Add public path information
// 	 */
// 	publicPath: bool,
// 	/**
// 	 * add information about the reasons why modules are included
// 	 */
// 	reasons: bool,
// 	/**
// 	 * add the source code of modules
// 	 */
// 	source: bool,
// 	/**
// 	 * add timing information
// 	 */
// 	timings: bool,
// 	/**
// 	 * show exports used by modules
// 	 */
// 	usedExports: bool,
// 	/**
// 	 * add webpack version information
// 	 */
// 	version: bool,
// 	/**
// 	 * add warnings
// 	 */
// 	warnings: bool,
// 	/**
// 	 * Suppress warnings that match the specified filters. Filters can be Strings, RegExps or Functions
// 	 */
// 	warningsFilter: FilterTypes,
// }