use std::vec;

fn main() {
    println!("Sum of common priorities: {}", part1(INPUT));
    println!("Sum of common priorities from triplets: {}", part2(INPUT));
}

fn part1(input: &str) -> i32 {
    let rucksacks = get_rucksacks(input);

    rucksacks
        .iter()
        .map(|r| find_common_character(r.compartments()))
        .map(get_priority)
        .sum()
}

fn part2(input: &str) -> i32 {
    let rucksacks = get_rucksacks(input);
    let all_supplies: Vec<&str> = rucksacks.iter().map(|r| r.supplies).collect();

    all_supplies
        .chunks(3)
        .map(|chunk| chunk.to_vec())
        .map(find_common_character)
        .map(get_priority)
        .sum()
}

fn find_common_character(strings: Vec<&str>) -> char {
    'outer: for c in strings[0].chars() {
        for string in strings[1..].iter() {
            if !(string.contains(c)) {
                continue 'outer;
            }
        }
        return c;
    }
    panic!("Didn't find common character");
}

fn get_priority(c: char) -> i32 {
    let ord = c as i32;

    if c >= 'a' && c <= 'z' {
        return ord - 97 + 1;
    } else if c >= 'A' && c <= 'Z' {
        return 26 + ord - 65 + 1;
    } else {
        panic!("Unknown character {}", c);
    }
}

#[derive(Copy, Clone)]
struct Rucksack<'a> {
    supplies: &'a str,
}

impl Rucksack<'_> {
    fn compartments(&self) -> Vec<&str> {
        let (left, right) = self.supplies.split_at(self.supplies.len() / 2);
        vec![left, right]
    }
}

fn get_rucksacks(input: &str) -> Vec<Rucksack> {
    let mut rucksacks = vec![];

    for line in input.lines() {
        rucksacks.push(Rucksack { supplies: line });
    }

    rucksacks
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn common_char() {
        let expected_chars = ['p', 'L', 'P', 'v', 't', 's'];
        let expected_priorities = [16, 38, 42, 22, 20, 19];

        let rucksacks = get_rucksacks(TEST_INPUT);

        for ((rucksack, exp_common_char), expected_priority) in rucksacks
            .iter()
            .zip(expected_chars)
            .zip(expected_priorities)
        {
            let actual_common_char = find_common_character(rucksack.compartments());

            assert_eq!(actual_common_char, exp_common_char);
            assert_eq!(get_priority(actual_common_char), expected_priority);
        }
    }

    #[test]
    fn part1_works() {
        assert_eq!(part1(""), 0);
        assert_eq!(part1(TEST_INPUT), 157);
    }

    #[test]
    fn part2_works() {
        assert_eq!(part2(""), 0);
        assert_eq!(part2(TEST_INPUT), 70);
    }
}

const INPUT: &str = "pqgZZSZgcZJqpzBbqTbbLjBDBLhB
wHptFFsHttHFLMDQDFTbbj
fVfvsstwPHwNwfNGfHWRSnlpClcJzCWCzddSrddg
bdgHbZJHgMHgJgJctDtVssVcpFtq
rNNQqBSzTcBPTDsP
GWNNrhGnNnWNzRfnRQRbhCdqHMbdmbZddbmCmd
BSBDzrSwrqccDDwbfcBjsRwggClslTRWGWGMFlsF
dnhVhLJtzNZdgCRlsTGWCRJG
ZHZdNzptLNtPhPdnprPbbDBrSqrSQPjbqD
rlSwlrGvwTTSwSggCJGQJdhVgJGQ
jcrHrMWfNHNzQgVH
WbfjmBMrBrrmLtqMbwwsPTvPpwvBPFPsws
NRNcHzbzbMRcNPjPrrlBPlbtBl
CZwVCCpWggqprwtlHlssHtPr
WpmLghCVCqCnmVTLnccRQvvQQHDhNQzzzc
NvGTmNGDJsrCmCWPHpCP
nqfVfnFQnZQfFqzMZBPtppcBPPCBptcrbF
fhRSSVfqMZZhMnQfjVzShNNlLvldsNDdvvljGpllDT
JGRNWRGJbGmCGRbLmGpqShhcQpQgCcncScSQ
FFdtjvvpvVFnQhhnQhgf
ltvjjtjHlzBtWRmNPLZRHLpH
FFCJFsvgLsjLgWzJFWJgGwBDbwnbwlDddqffnjnlnd
pTpTMQpMZHQhZQpHPZMmTMlwhDNNddbnDqdDwwlbVNVd
SHpmrHPZHQpmvFrqsFvgCsRq
TtWpWhQlVZrVptJhtrtdbLPDPbjFbCCWCvFFbLSN
zMGlnlsgSSvjjGSF
msznlgcwMnRwznmBqTZQJrddhfQJtBJtZQ
mwTwLftmqqSHWfCfLHjWftBthNNNVrlcFRVNrNrRTrMgrRNg
pvJPJQPGPPzbpVpVlMBVchFl
bzQPQbQQdsDZPDGJnBGnPGnjHDttqqqqmCjftLCmWmfftC
ZNpfdHcccTfdwfMFNjBttMgMbBnvlvjBmB
QVPsGzhbszRLRrgvtgjWgljlRtgt
VJrVLsSrzLzGPChVGzbrrfpTSHFfDDHSdpZFfHpDTZ
NPpvDbmbsmdbNvQvDdWQpmWSnnQCLBnCcQSCnnLlwCBlZz
jggrtGTFhtGfjhDVjrjgMftFwnZcwwBCnzzVwBBwSZcVwLSw
JfHftHhgftgFJWPdPDWRPDvPJv
ZSLLZJGglDSVNDGGGgGgngGmHrfLzmHvvjfjwLhHvLfHHr
QqFWszFMTQFdFPMqBmWBHvfhCwCjhHff
pTsdppTMPtqqdbnlNVzJVbSSnbZR
gBqDccrrJDwmpTWHHTdWMPWWZFHF
RNfnfSwRjlLSWjQMHWvQZtvH
GRLbnzNnzVRLCqhwzwBmJrmc
CcGnZGnGlRncsspmFmmcmGRJJzCDTzjLBSDfqjwDDzDLDB
hHrNdPWhrbPdhPgVWvvrgWdfwjfNzSqfqjLqzfBTzJzJTz
HHQhhvMWbbdRswmlsmwsQc
rrwhpZPrccRpQdcFDJNqhtqtqMLhqfMh
lTtTllgbzTlJsmDMvbLbsf
VVWBTgzlzgGnngrtQRQRtCtwZRQB
HGnGvVdLhlFcmvPWmT
jwBLqZgjrtjqmFsQTscPQs
ZBNZtwLwztLpMrfZBLMdbdnfSRVVfnGbnfDVGh
LmBBBzQrBgBhmmggmtdVdhJNMHNdhsHNDd
ScSZbRplCcMnSpvCfCCZcpPwtJPNtHPHNVVNtPddwlNH
CpvpZGfnffSpnvRSSbcfScQBWrMWmjrmFFBzTGQWjMmL
sljSjSgsjcCLllsjVgSjCtspQwvNNhdFwFQvwpbtmhwhpN
hDqqWzGRHHfRrJnrWrfWfHBdFmBFmBbdBGBpwmFdFpww
TDnDHZrWWHhTjPClClSP
wGNQGQDGjqqmwHHs
MWvvrzgfsdWsvMrSdqJqcpgHgnqLFLnjpH
fvsPMsPdrTZhChNDhbhPVN
sbMgDDtttVvpMtcJsgcGGBBfGLBSLclQTGPS
CWHWzhhRRHjqRmSGPfBSzJfSBnBB
HjjRHWFWhRRwHNmCCRHhhFdNDNrdptMstJvsbMDtVptd
RnSwRsLsnSswjDDDBJPrJv
cpzCzlczHTJVQhvBQlDVDj
WmGzqTmHSsffqqnJ
vQSPHMwpmpQMLGfTPVLRPRVP
hsWhnncsJqbGjGfcfBMMRR
sqdNWqqghbsJslgsJqgWllMWDppSvNCHQHDSSHrHrHCSvvCQ
ZWWnWMmmndQZmffcdZcmssQqrsptVwwTtQHTCTHH
SvvrPzvvFDzGzTszpGGwHT
vrRLjjrPhLjrjPDSfdcMZnmdcmJcfcRf
HpqWhDJjzmcTSbmMBVBb
nCzfLtFnZZrcbcVVfTBfsc
FtRFzgrRtnRzrFwzDjljpjgHNJDlNlhN
gtNRRSSrRmjshHmm
PQDMwPwMppcQQcvCFlhLhGmjflctlnHGjf
QwDMFFdtwFDQJZZZNqSqJSqBgBqTNJNg
cddzbbzQflTDcDfRbcfbJVsplVsChNghHNsSsVpn
FBWFWjFFCjWPBSPPJsVpppPSVH
CrCwvjWvmqmvrBvFwZRGQQDDcGTcfbddZdRc
ZBQqdGLFmmzDmTZz
PvrVMvGgWmwSmllglS
NrfGnvWWPhfpspsGvLJBsFBbqJCbdQcLBq
DrwTrlfGThhQTpDdWSWgdgwLLgBSZH
bqbPRVRmzJCLWSgCzCLH
jRtbNVtNjNqRqsJtbjbMDQHQGlchfQpfsTcHpGHr
tMnRcnpDcZtpQDSCCsGGHvcGPGqGsr
mzJmjWJNlbfmbhzVCCPmZrCZqPGCgZ
JWhzjJBdWnMLZtSBTw
tvdLttzvtHLztnQpssdTPbMqbqMTdTss
jhSRGNjjSjhSDCNhRgRgclNjmfZTPnbTMqJJfqqsbMflTfMs
WnCjcjDRCChSNSCNDjNhGVDtQvVLHzFrpFwFrHFQQwQpzp
fLbLLLLQhVPhBVmDwmCfwsdwwDps
GNtctFTSrrJqGGpHFcTJFTwsslwmlmWsdsqRRCmqwdWs
ppctFTTSgHcSrGrrTGFcrrnhhQbPLbQZgzLvQQVVvZvLhv
mBBWnnBbBCtssmRThRDllR
wfwFQcpHFpddFrwpGcHSHdcjQZZlqqDTTjZqssRhWllssj
dWfrcSGFpgrSzFgMbCPNPLtCtVMV
llLlGLJJMjJMGVSvVMSLRRHvjCZtgZccttnmbCtdCcmCCztn
sBQNqPhsrrqrrwrsppsHswsZcmnNCzdZtbgntcNgcctnCt
WWFBBsPwpWPwBBHpFFrWGRMRLlJfJVMJRJLWSJ
vgMvQnPMntnSQPSgMvSMpNJfJDNNRpfZmGQmbDND
HlbjHHBLjCHGZGpfJLpfwm
qdHWqBbbbjrTzdqFqssvtPMSSFFg
TGDfDHSgtTzPPbnCtnNtVn
WQrWMFpMWMQbCVNPRWVWWv
pdpMMrhrprQshlMFjZpdjZMgmlGJJGlGDBmgmHgmJCSBHG
zWWBjZZjWPFFPPnBCVdsqmnCdSLn
rJvpbvbpGgTGrNJGGpRRhsCqSsLhnsmTVnSLTLHh
gNNNJDbpvGNfvNSDPPWQWWPZZWtjlQfc
RRVbWWWvvZVWmsFSsDNbHsDSsg
CrTwJQJpJpCCwvlJQTTPsfzDgfwNhszfszFhzFDh
ttQJtvjpPvcqTllJTPtPRGMGRGLWdVZLVZjWdMjj
NnPCTQWMMQNNNWwWnMzpHczzsZcCscddHdGs
mqRgqqVlLgqmfVzcGpzzSHGZcgcz
tjmZjZmhqftlJRJhlTMPTPQbrPBBWnhnnT
hvTQqpvTqjvhpjnCqmCnSDSFDWFFLSSSWDnSVL
tZwGgsfPcltgcZltRgNSDSSSSldmWMLWFVHd
GrZtwRPbGwwPcGRsZGtRtgQJJhCjpzmTBTvJzJrjvzQp
rwmwqDWwfDtztnFGBB
LPdpdVcdPGvPVgZsPtlhTTtthHBhHF
dRdCjvpCRpjvCMZgvLgRVJJSWMqmbwQJbMWGWQNbbQ
CMCcMcDGzBGPmBmznTNbnGbrswNTwTvN
SSHVWZphqWWJJzNsbnFwFVNjbz
flLQqHzzgtQdcDRB
mdzvFtllBgFttGnvfMwMVRRZCThSNZVhMd
pDTrDHjWWJPqjDjDSMqNwSZRZhNSRNCZ
jpcTpQPWLLpDTLcTrPjPDcjzzFLFzvgLzlzfvGFgfmgFzF
fQVVPzBpFVVrtrsJ
PldSLNSmWwMCcCMMcCNN
mSPlldllmPdRnLRwmbnLwmwvTjBTghTHQjfgjpZHpfHHfZbZ
pmfMcfprMqMrZZJcMZMGWTsFCVCTVPPsVTWCGPDP
vrvvvLRbBNNBbvBbjBHbQhgDslPTWsPTlFDsFTFwTWlDVQ
hgjznNBjHHgrhRHgrRLRnRfSSJmdqMfffzqJptdmmmdd
nRnPlCRPWPMFqwPLwq
tBGfbSbHtBVQgrbrqfTFFLvTNLLNGTGMLdws
bgHVtBDtqnqqlJRD
SdSJrHssFBSVsNtMMdRWnTRhRl
vcvfDvgvcwvFRlbnwWRlMhtn
DDDqcqFZQPgcgcfvDjLDfVrsSVrHBLJVpLpCSppGpS
gJGTFLTdrpLdBcWBvnllvlMvMC
RRqbbQhwNZZwQRPrSZWnvHSZWSvSZC
fQDNRsrsQzfbDrbsqwdtpgJVjdJdpVfJFLFF
DzWqFvqpqFSCSzGRGmwfntGjmR
cbhZNJQBtgMHJbJcNcrmfhrRrswmfRwnVrhG
bJNgbNdJBBPMHbcMNMWWvSFpDLFvCStqpLdv
sLsHTsTbRLRwqssHwHjFrPDwJDppzFDJmmcrPJ
BnZGBlMZnQSgSnvVSMmJzPDCzFcrLPPJmpDG
BgBffVLhQLgvnBRRssfqdfHbHdNT
HRPVmjqBqVjVRRPmcPmJjbDgLDDshbfRLlfbfLbhlL
rtTzSMSMFpTzfgDzzgsLfLHZ
rNpGpSSHwMTrrdHGNtTPmVjnGGjVjmBGmmBjJB
DBqDQDQHSFlHsFnN
MfLfwwLMWGLrWMMnpSlsnGJJlbFVjV
gRhMZzhrFLWQvTPqTPcvvh
NwwsHwtnFCtzcPdbvrQbBqclQq
VmZLLTLfVpwMBrVVqqMM
mgJDjTgWgLWDppJZJTWZmSRzCtRHhGGwHNzshGFFCSRt
RGCCDRdFZdRCMzzGCDGCmGHMfqbNNNLQLfFqnnqnNQqVPnQn
glgcrwrJjJccBwdSfnSnVqrqQVVnNq
jvtBsjstgstjltBcWzTGGddHTWDTZCmDGm
HJHGZZHnctSSDhZtmZ
MjjQFSvQlRjSdRqdqvVSqCCPtpRpPPDfDmfPbbpphC
SNsWqMNvFFqdqVMgwwBHrGHnHgcWTJ
jBcbjSmSBbbCcPcMjmbzFPhDMDfrfGRhGQRMnGQfdrDh
wHlqwlqpwZqcwVlqHtJVJLTdhndTDnhffftTGDTTDdTG
JNllcwpZZlpZJjNzSzSCNjSmFN
FhwRPzmPWmQQmwFPGGMGGRPnRHHVfDbvJlvDlHSvDTDfVHbD
NpjcpCdqpZZvwvJVfDdDHT
twZtqrBrBQBMBPGn
fBFGjbLLFblmbWFmVfBvrvMdMdncnrdNbdQNTr
shZHHRZhtsqJZhHhgZzgJzVJrrSSvrMdMQrNTvMNJQNrdn
szHwgtHtwPzPLpVFpVPLlfLC
mrsrtrWjljjjvwwgNnZfDHJDqTqrHL
FccMPFQcpczpdMPhMqJngNfqfnFgDnnFfg
BdMpdcDPcpjBmlBmVWts
VvwTTlfVlblwwSsbfTdzVqjhzVjpjjqjqpzV
rwCWFGmJrNCmMRHmwRFPmHQQhBLBzdLqBjhLBHZdQB
rFwDrMNRJDJFPRmCvcTcbDsvstTgfTsg
zhRzdRRChHCFGPDRvWRWvWvHpZpscrrmrZrJcmspJmJZFfpM
wQqLtQLtnjbjVnVjbBgjbBnbMZMZJlVpZfJprsMprmGZZZml
jQjjNBLLwjtQBtwwdGGDCHhNzzWDzTPD
DzzQnCMMznFnCdnFFlHtlmhVRtmVVmVhSF
PWrPPRTfLJJtfbtBfV
wsrggZsTwTTWGvDppQMRjjMCjMZp
fTjzZVTlbffCMvjgMpSFWBNBWSFsvBsNNccF
nJdwdPRQqGqbGJQbmmQQhRSBBBSsPPHWNSWFBtDNBsHH
nnQhwwQGdLqqwnZbpfjMfzpzLbLj
jgTgCwgjMgGLhvRrHrHwhvhV
bqSsSsZFZBlFsBlTSppVvVvnVHHvHnhp
qFlbPbFFsWFsBlFWbsbsmzTcMjLmtfcCmcWtgzgm
rrHbfBLbfMcghcmrcCzg
RDStDtvdZRQdJSQWWWdvFSgNvVcnghhmnnzhVPhczPch
ZtJpJttSZStwtttFDQmLGTlqMLqGfwTTGLfTGG
MrfLWwfBwgghvLmNvmHHHGGQHQSSscscVvTV
dDjZjDPJtFRzjdTTsqVjTpqHsGrT
JbFtlbPRJCzffBrgnlMWmg
ZFsbbVLLdZppLFpcJjCCQJlGcQCMZq
TwRtRBdBClCTGlcJ
rwBvBzDvwNNDHLHzfHssdHhS
gdhgftTNGTbpqJqjjgRJ
lcBcMLFzMzLFMzFzPjRBQjQPQpSqhpbp
mzzmZHZZnZwLhtGfddVsNCCnGG
lblbPGSGrTLRwqZLvP
FffCCFzFCWzzvmjRJnRTnZZNJCTqCR
gdhztVjhHMsGvrGVVB
ZJZjJGHZhDJRFJHjDZjhPNFgFmrnVmgVVzVBscnzSg
bwlWtMwtbqdCvlQCplmsqgnVVScnVgmnmzNs
WMWltTtvvCdwCCRPPfTHGcJDfGZL
svqQJLvSSZrZZZCFCBDPDCMTDpPwMWDPCwRw
GnlnGbdldjhzzhpPDTWjmtMwPmWW
HzVbGnnbchblbnbzcQTZBZrQrFSHvLBBJv
MmgMmVpcRDlvbvpHJF
GSGTLTwhwwhzQqTqwjFlbdvdbrlrbrrDnDvHlQ
zLNNNtwGFCMsWsCWNR
tSTDDDftSqSsTDnTtCWNrbFsNJJvbzJbvJ
dhRdVHdMGRgPJbjNPbzgvr
VllQmQdhRHLhhHmLlGzSqSQDDcDBnnBnqDfSct
zBzJWZBLZNNGLsbTvLbmbT
QdtQwfdnPdPTbsRQGhRvbl
pgtPgPjVDnpVnDtPTFFrJJTBCcpcrpCW
GnWMfBfdCGMbjRNpnzvvNLRNVv
FShJDJJscwwszNjvNjNNqZ
tJmccwlcFlFcHlPcHFfdrbBGBGfjCGTfBCPf
GhlcQsZNQZWhpcGhwlPmqnnqnjJjLRnqzJsJLJ
VTMtTtDTbvbMTfvdJqngjmqzdjmJjCLm
vHtbHTDBFvffBPGwLLZBQNNl
bDphJrpbpnBbDrdBvJdDFBMtMlfgtsFSstfGPPgggPGP
RZmNjTZQNVHQHNGSgMsfPlShSs
VmchTLZQLjVLjmTVmQVhTmwVrWJqbDqddBrpnWbvnqrqcnJB
jWWgThWtgSvSSWlJtlShllPcHVnJHPbMHPcPVPbVZrHM
fGdfRsRdNwfRQhnpcZdVhVpbPh
fGhwNBqNjqStFqtj
TSTBrSDlQlTDrrQclrBSLffPvcfcdVjVMGGPLjLL
qnbnbngFGhhhPfjjVffjff
RRWbmgpnmqlrCwwSrwmG
mZZTsdBZVZBZLVHdFmsNnCrCVQQbWvWjWNCnbg
QSffDGwGGrPGWrgN
hflwzltflDpMpDSllcMDhSShdsdZQdLZZdHTssZzmqLzFmLB
LLRJRshLfsJfWnLBTlTBlFzNrnrBBl
qmmVwmdHqmqGHZdHbbqSScdZQTjjpTFFVBBrlDrzDFBTjFjF
wZZwmcbvHgqTmGccmvdCLhCPJsJCPWgMLPtJsJ
TWbbbNbJJjJbqTjtJJjTQCtnGSBndMGCcSZSQwCB
mcfRfrcmrDRrPsdQSGZQGnsSQMnB
DDRLDRDFPpgmpcgPghpfgvRTjbhTVjHljJjzlVzVTlbHll
rPlPrPllBGgJgdJfHgfjJt
pppZVfFDWssMfFVVFMpsMMVmHCRLdcZCRtvLRdCtCJdHRttH
mDMfDFDmnMMmsMFznDFpzswbNbPGwwSGBrGrhrTzThSl
qDNFfCCNWLfWWhqhDGPMMZVwgpCpMbJwJCvV
RdstRRvdtmtPVpppVbVtrp
zzncSRdsTdQTczQBsLvvHNhDWGjDHNLDSG
bNNpcfJcCtNpHFsJsGGjLGzmLjLmGzlFGW
qwqZdnQnQwnhhzmnMWjmNlMzLr
qhwwQSwStJbHNftS
WlfWSwDftzRltBWVlRDlsmBJPcsZPmcJnmPmFhrn
dLQbQbvGTddTvbjQCbLbhmCrZZPPsshPPPrJZrnF
QgFjQHHbMvdRMVllSqfSlf
MDPJBWWPggVlPVDMSljdZNNpwjwbHZpNbDdH
mGmzcThGrtntHhthzGctRbNRNwRNzZwfdZpjpdRj
ThtcvvtThFcnqFQSHgBSVJll
hVqhFLBngHVFtJjtLCBJVSbbPNNbSmfLLTSNSrrLTb
vsdZZpvQdczlMdMvzlcvvdQprbGGTfSbWmzPTgmmGTbmmfGP
vgRZZMgwdgsQZdMBqVhjhJqBhJtRhq
bgFQbMMbTbQhghddFTFGnmSmsNdzHvzSSzlcHsls
fZDjVtfZLqwpqtCfCjCjlvqScrvzqHSzszzSnczr
jpftjCfWCjCfCRZZlpCpjRWQBRTQQbgBBTMbghgbbPGJBJ
VZZrbBVwbbbVVvgbntnggNRJqRRNNccMcNqJcJ
jfDPfDdGGhDGfGFPCcZQqMpRNJhqTcMc
LPfffPHGPDjPFGWSdHPFjWtlBlwvlwBlbtmLltsmvtZb
TTfJDfrJTSrHMcVMJDTfMcMDBQBPwnPlznPszFVBFgzFgnsn
CqtmWNNGBPzwbbwm
htdCthhWGtWWGNZqcZpJjwwHHHMcHZDM
VvjQjQCZLbbSbTPpSHtFzsHzppMfzz
DJrJWBcDcWJWmmcgGRGRGWGDzHHwzdfRHpMztMpfdFdFdzdM
DmBgGDqJNhGcccWmcZLjTPLVLTQhPtvvTZ
qfhvwNDQqwDGdGZZGwPTTw
STsJgsRtJMZPjlsmdpbs
SCTTHTWHNVfHQqqq
djCDgllgjJjDRRNgRlDdBgtpQHfhQTrLLrGBtzrQhpBH
SVcsMGcPSbqSPmLTPHHQTHrftPTr
VcWnsScqSScWcZbMMcSVGbNNlgDRlgCCNgwWvvRJdNdj
mgPllfRgvNmPGQGGsmQNWlpFtnBPFShncTFShtFShnjS
tLLzMJJwwbbdrrMLqLVJMzVZFnpTTFpnCSSpShCjBJhpThph
HMdVwbbLMbDMDVlmDsgtNtNRfgsm
hNsgsgzNZRghPhZBdssPQfzDmQSmmzQGJWzfCDJJ
bblVHvvHHTljwFCfGrvmfmmJBmGQ
THMMFVwqTPRdZptMBP
QvcPGSvQLjmcQWSGWWGjLCNhhqpCdBCNCbJNdVWpCh
rwtLlzZggLHnHlwHRDdVqBbCdqqVVhbqVnVh
zRDzwRrwlRlRTgrDtllmQGLcPjGLccFmTcGSQc
RWlgQlbcWBwzsJggTfhh
GrnLjHLjmLjjGSLjSDmfJJpfThhfSWJPqJqhwz
vLvDDnDNrCVjCmNDbFlBVZdVRQlRbWcb
mTlwFngwmlLlvsmLHmHsLJhJFfcbdpbNcjCNCbpccb
tZRzBRzBBRQzPqGRqrVQtjjfbCMcfMfCMMjVjfCJNd
SDBBPtZZTdnnwSvg
nddNNMMPNBnBNnBTQSShlSHghlDHBr
VcccVmqJsJsjlTmzTDggmHHT
VqLtFCqFJfVtVjsNgPNNMMWNwgtNvn";
