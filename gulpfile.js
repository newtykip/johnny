const gulp = require('gulp');
const uglify = require('gulp-uglify');
const typescript = require('gulp-typescript');
 
gulp.task('build:ts', () => {
	const ts = typescript.createProject('tsconfig.json');

	return gulp.src('src/**/*.ts')
		.pipe(ts())
		.pipe(gulp.dest('build'));
});

gulp.task('minify', () => gulp.src('build/**/*.js')
	.pipe(uglify({ mangle: { toplevel: true }}))
	.pipe(gulp.dest('build'))
);

gulp.task('build', gulp.series('build:ts', 'minify'));
