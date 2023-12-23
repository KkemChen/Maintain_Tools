import { defineConfig } from 'vitepress'

// https://vitepress.dev/reference/site-config
export default defineConfig({
  base: process.env.BASE_URL || '/vitepress',
  title: "运维手册",
  description: "A VitePress Site",
  outDir: "../dist/vitepress",
  themeConfig: {
    // https://vitepress.dev/reference/default-theme-config
    nav: [
      { text: 'Home', link: '/' }
    ],

    sidebar: [
      {
        text: '常用Linux命令', // Linux常用命令
        items: [
          { text: '基础命令', link: '/LinuxCommands/基础命令' },
          { text: 'docker常用命令', link: '/LinuxCommands/docker常用命令' }
        ]
      },
      {
        text: '常见问题解决方案', // 常见问题解决方案
        items: [
          { text: '流媒体取流失败相关问题', link: '/IssueSolutions/流媒体取流失败相关问题' },
          { text: '来邦对讲失败相关问题', link: '/IssueSolutions/来邦对讲失败相关问题' } 
        ]
      }
    ],

    socialLinks: [
      { icon: 'github', link: 'https://github.com/vuejs/vitepress' }
    ]
  }
})
