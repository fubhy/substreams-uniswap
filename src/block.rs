use substreams_ethereum::pb::eth::v1 as pb;

#[derive(Copy, Clone)]
pub struct Block {
    pub ver: i32,
    pub hash: &'static [u8],
    pub number: u64,
    pub size: u64,
    transaction_traces: &'static [StaticTransactionTrace],
}

#[derive(Copy, Clone)]
pub struct Transaction {
    block: &'static Block,
    inner: StaticTransactionTrace,
}

#[derive(Copy, Clone)]
pub struct Receipt {
    transaction: Transaction,
    inner: StaticTransactionReceipt,
}

#[derive(Copy, Clone)]
pub struct Log {
    receipt: Receipt,
    inner: StaticLog,
}

impl Receipt {
    pub fn logs(self) -> impl Iterator<Item = Log> {
        self.inner.logs.iter().map(move |&log| Log {
            receipt: self,
            inner: log,
        })
    }
}

impl Log {
    pub fn address(self) -> &'static [u8] {
        self.inner.address
    }

    pub fn ordinal(self) -> u64 {
        self.inner.ordinal
    }

    pub fn data(self) -> &'static [u8] {
        self.inner.data
    }

    pub fn topics(self) -> &'static [&'static [u8]] {
        self.inner.topics
    }
}

impl Block {
    pub fn make_static(pb: pb::Block) -> &'static Self {
        let block = Block {
            ver: pb.ver,
            hash: pb.hash.leak(),
            number: pb.number,
            size: pb.size,
            transaction_traces: StaticTransactionTrace::make_static(pb.transaction_traces),
        };
        Box::leak(Box::new(block))
    }

    pub fn transactions(&'static self) -> impl Iterator<Item = Transaction> {
        // TODO: Filter for only successful transactions.
        self.transaction_traces.iter().map(move |&tx| Transaction {
            block: self,
            inner: tx,
        })
    }

    pub fn receipts(&'static self) -> impl Iterator<Item = Receipt> {
        self.transactions().map(|tx| Receipt {
            transaction: tx,
            inner: tx.inner.receipt,
        })
    }

    pub fn logs(&'static self) -> impl Iterator<Item = Log> {
        self.receipts().map(|receipt| receipt.logs()).flatten()
    }
}

#[derive(Copy, Clone)]
struct StaticTransactionTrace {
    receipt: StaticTransactionReceipt,
}

impl StaticTransactionTrace {
    fn make_static(pb: Vec<pb::TransactionTrace>) -> &'static [Self] {
        pb.into_iter()
            .map(|pb| Self {
                receipt: StaticTransactionReceipt::from_pb(pb.receipt.unwrap()),
            })
            .collect::<Vec<_>>()
            .leak()
    }
}

#[derive(Copy, Clone)]
struct StaticTransactionReceipt {
    state_root: &'static [u8],
    cumulative_gas_used: u64,
    logs_bloom: &'static [u8],
    logs: &'static [StaticLog],
}

impl StaticTransactionReceipt {
    fn from_pb(pb: pb::TransactionReceipt) -> Self {
        Self {
            state_root: pb.state_root.leak(),
            cumulative_gas_used: pb.cumulative_gas_used,
            logs_bloom: pb.logs_bloom.leak(),
            logs: pb
                .logs
                .into_iter()
                .map(|log| StaticLog::make_static(log))
                .collect::<Vec<_>>()
                .leak(),
        }
    }
}

#[derive(Copy, Clone)]
struct StaticLog {
    pub address: &'static [u8],
    pub topics: &'static [&'static [u8]],
    pub data: &'static [u8],
    pub index: u32,
    pub block_index: u32,
    pub ordinal: u64,
}

impl StaticLog {
    fn make_static(pb: pb::Log) -> Self {
        Self {
            address: pb.address.leak(),
            topics: pb
                .topics
                .into_iter()
                .map(|t| &*t.leak())
                .collect::<Vec<_>>()
                .leak(),
            data: pb.data.leak(),
            index: pb.index,
            block_index: pb.block_index,
            ordinal: pb.ordinal,
        }
    }
}
