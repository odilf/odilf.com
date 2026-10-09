import { defineEnvVars } from '@sveltejs/kit/env';

export const variables = defineEnvVars({
	ODILF_BLOG_PATH: {
		description: 'Directory holding the blog markdown sources',
	},
	ODILF_MEDIA_LOG_PATH: {
		description: 'Directory holding the media-log markdown sources',
	},
	IMMICH_URL: {
		description: 'Base URL of the Immich server serving the pics album',
		schema: (value) => value,
	},
	IMMICH_ALBUM_ID: {
		description: 'Immich album to source photos from',
		schema: (value) => value,
	},
	IMMICH_API_KEY: {
		description: 'Immich API key used to read the album',
		schema: (value) => value,
	},
});
