// 引入 API
import { ApiPromise } from '@polkadot/api';

async function main() {
    // 创建api 默认链接本地测试网络
    const api = await ApiPromise.create();

    // 订阅系统事件event
    api.query.system.events((events: any[]) => {
        console.log(`\n接收到 ${events.length} 个事件:`);

        // 遍历事件
        events.forEach((record) => {
            // 解构 phase, event and the event types
            const { event, phase } = record;
            const types = event.typeDef;

            // 打印事件所属 pallet和call
            console.log(`\t${event.section}:${event.method}:: (phase=${phase.toString()})`);

            // 遍历事件参数和数据
            event.data.forEach((data: { toString: () => any; }, index: string | number) => {
                console.log(`\t\t\t${types[index].type}: ${data.toString()}`);
            });
        });
    });
}

main().catch((error) => {
    console.error(error);
    process.exit(-1);
});